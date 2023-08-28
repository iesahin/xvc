use crate::Result;

use crossbeam_channel::{Receiver, Sender};

use std::collections::HashMap;
use std::fmt::Display;
use std::io::Read;

use std::time::Instant;
use subprocess as sp;

use xvc_file::CHANNEL_CAPACITY;
use xvc_logging::watch;

use serde::{Deserialize, Serialize};
use xvc_ecs::persist;

use crate::XvcStep;

/// Command to run for an [XvcStep].
#[derive(Debug, Clone, PartialOrd, Ord, Eq, PartialEq, Serialize, Deserialize)]
pub struct XvcStepCommand {
    /// A shell command that will be run via [subprocess::Exec::shell] in [crate::pipeline::s_waiting_to_run].
    pub command: String,
}

persist!(XvcStepCommand, "xvc-step-command");

impl Display for XvcStepCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.command)
    }
}

impl AsRef<str> for XvcStepCommand {
    fn as_ref(&self) -> &str {
        self.command.as_ref()
    }
}

/// Used for encapsulating a process and its outputs. This is used to associate steps with running
/// processes.
#[derive(Debug)]
pub struct CommandProcess {
    pub environment: HashMap<String, String>,
    pub step: XvcStep,
    pub step_command: XvcStepCommand,
    pub birth: Option<Instant>,
    pub process: Option<sp::Popen>,
    pub stdout_sender: Sender<String>,
    pub stderr_sender: Sender<String>,
    pub stdout_receiver: Receiver<String>,
    pub stderr_receiver: Receiver<String>,
}

impl CommandProcess {
    pub fn new(step: &XvcStep, step_command: &XvcStepCommand) -> Self {
        let (stdout_sender, stdout_receiver) = crossbeam_channel::bounded(CHANNEL_CAPACITY);
        let (stderr_sender, stderr_receiver) = crossbeam_channel::bounded(CHANNEL_CAPACITY);
        Self {
            environment: HashMap::new(),
            step: step.clone(),
            step_command: step_command.clone(),
            birth: None,
            process: None,
            stdout_sender,
            stderr_sender,
            stdout_receiver,
            stderr_receiver,
        }
    }

    pub fn add_environment_variable(&mut self, key: &str, value: &str) -> Result<&mut Self> {
        watch!(self);
        self.environment.insert(key.to_owned(), value.to_owned());
        watch!(self);
        Ok(self)
    }

    pub fn run(&mut self) -> Result<()> {
        watch!(self.environment);
        let process = sp::Exec::shell(self.step_command.command.clone())
            .stdout(sp::Redirection::Pipe)
            .stderr(sp::Redirection::Pipe)
            .stdin(sp::Redirection::None)
            .env_extend(
                self.environment
                    .iter()
                    .collect::<Vec<(&String, &String)>>()
                    .as_slice(),
            )
            .detached()
            .popen()?;
        self.process = Some(process);
        self.birth = Some(Instant::now());
        Ok(())
    }

    pub fn update_output_channels(&mut self) -> Result<()> {
        if let Some(p) = &self.process {
            if let Some(mut stdout) = p.stdout.as_ref() {
                let mut out = String::new();
                stdout.read_to_string(&mut out)?;
                self.stdout_sender
                    .send(format!("[OUT] [{}] {} ", self.step.name, out))
                    .ok();
            }

            if let Some(mut stderr) = p.stderr.as_ref() {
                let mut err = String::new();
                stderr.read_to_string(&mut err)?;
                self.stderr_sender
                    .send(format!("[err] [{}] {} ", self.step.name, err))
                    .ok();
            }
        }
        Ok(())
    }
}
