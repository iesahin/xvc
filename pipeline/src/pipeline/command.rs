use crate::Result;

use crossbeam_channel::{Receiver, Sender};

use std::collections::HashMap;
use std::fmt::Display;
use std::io::Read;

use std::time::Instant;
use subprocess as sp;

use xvc_file::CHANNEL_CAPACITY;

use serde::{Deserialize, Serialize};
use xvc_core::persist;

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
    /// Environment variables injected to the shell that runs the command. This is used to pass
    /// added, removed items in certain dependency types.  
    pub environment: HashMap<String, String>,
    /// The step that this command belongs to
    pub step: XvcStep,
    /// The command to run
    pub step_command: XvcStepCommand,
    /// When we started running the command
    pub birth: Option<Instant>,
    /// The process that runs the command
    pub process: Option<sp::Popen>,
    /// Channel to send stdout to
    pub stdout_sender: Sender<String>,
    /// Channel to send stderr to
    pub stderr_sender: Sender<String>,
    /// Channel to receive stdout from
    pub stdout_receiver: Receiver<String>,
    /// Channel to receive stderr from
    pub stderr_receiver: Receiver<String>,
}

impl CommandProcess {
    /// Create a new CommandProcess by creating channels and setting other variables to their
    /// default values.
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

    /// Add an environment variable to inject to the shell that runs the command.
    pub fn add_environment_variable(&mut self, key: &str, value: &str) -> Result<&mut Self> {
        self.environment.insert(key.to_owned(), value.to_owned());
        Ok(self)
    }

    /// Start executing the command in a shell. Updates birth and process variables after
    /// detaching.
    pub fn run(&mut self) -> Result<()> {
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

    /// Collects the output from process and sends to output channels.
    pub fn update_output_channels(&mut self) -> Result<()> {
        if let Some(p) = &self.process {
            if let Some(mut stdout) = p.stdout.as_ref() {
                let mut out = String::new();
                stdout.read_to_string(&mut out)?;
                if !out.is_empty() {
                    self.stdout_sender
                        .send(format!("[OUT] [{}] {}", self.step.name, out))
                        .ok();
                }
            }

            if let Some(mut stderr) = p.stderr.as_ref() {
                let mut err = String::new();
                stderr.read_to_string(&mut err)?;
                if !err.is_empty() {
                    self.stderr_sender
                        .send(format!("[ERR] [{}] {}", self.step.name, err))
                        .ok();
                }
            }
        }
        Ok(())
    }
}
