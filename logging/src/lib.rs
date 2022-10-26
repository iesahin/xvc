#![warn(missing_docs)]
#![forbid(unsafe_code)]
use log::LevelFilter;
use log::{error, info};
use std::env;
use std::path::Path;
use std::sync::Once;

#[macro_export]
macro_rules! watch {
    ( $( $x:expr ),* ) => {
        {
            $(
               ::log::trace!("{}: {:#?}", stringify!($x), $x);
            )*
        }
    };
}

// Logging

static INIT: Once = Once::new();

pub fn setup_logging(term_level: Option<LevelFilter>, file_level: Option<LevelFilter>) {
    INIT.call_once(|| init_logging(term_level, file_level));
}

fn init_logging(term_level: Option<LevelFilter>, file_level: Option<LevelFilter>) {
    let logfilename = &format!(
        "{}/xvc.log",
        env::temp_dir().to_string_lossy(),
        // chrono::Local::now().format("%Y%m%d-%H%M%S")
    );

    let logfile = Path::new(&logfilename);

    let mut dispatch = fern::Dispatch::new().format(|out, message, record| {
        out.finish(format_args!(
            "[{}][{}:{}] {}",
            record.level(),
            // chrono::Local::now().format("[%H:%M:%S]"),
            // record.target(),
            record.file().get_or_insert("None"),
            record.line().get_or_insert(0),
            message
        ))
    });

    if let Some(level) = term_level {
        dispatch = dispatch.level(level).chain(std::io::stderr());
    }

    if let Some(level) = file_level {
        dispatch = dispatch
            .level(level)
            .chain(fern::log_file(logfilename).expect("Cannot set log filename"));
    }

    match dispatch.apply() {
        Ok(_) => {
            if let Some(level) = term_level {
                info!("Terminal logger enabled with level: {:?}", level);
            };
            if let Some(level) = file_level {
                info!(
                    "File logger enabled with level: {:?} to {:?}",
                    level, logfile
                );
            };
        }
        Err(err) => {
            error!("Error enabling logger: {:?}", err);
        }
    };
}

// Used to differentiate between different types of outputs.
#[derive(Clone, Debug)]
pub enum XvcOutputLine {
    /// The output that we should be reporting
    Output(String),
    /// For informational messages the user doesn't usually follow
    Info(String),
    /// Warnings that are against some usual workflows
    Warn(String),
    /// Errors that interrupts a workflow but may be recoverable
    Error(String),
    /// Panics that interrupts the workflow and ends the program
    /// Note that this doesn't call panic! automatically
    Panic(String),
    /// Progress bar ticks.
    /// Self::Info is also used for Tick(1)
    Tick(usize),
}

impl XvcOutputLine {
    pub fn info(s: &str) -> Self {
        Self::Info(s.to_string())
    }

    pub fn warn(s: &str) -> Self {
        Self::Warn(s.to_string())
    }

    pub fn error(s: &str) -> Self {
        Self::Error(s.to_string())
    }

    pub fn panic(s: &str) -> Self {
        Self::Panic(s.to_string())
    }

    pub fn tick(n: usize) -> Self {
        Self::Tick(n)
    }
}

impl From<&str> for XvcOutputLine {
    fn from(s: &str) -> Self {
        Self::Output(s.to_string())
    }
}

impl From<String> for XvcOutputLine {
    fn from(s: String) -> Self {
        Self::Output(s)
    }
}
