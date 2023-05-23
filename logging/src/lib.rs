//! Xvc logging and output crate to be used in output channels.
//! Xvc uses to discriminate outputs of various types, (Info, Debug, Error...) and can use
//! `crossbeam_channel` to send these separately.
//! Downstream crates (xvc, xvc-file, etc.) use this crate not to use stdout, stderr directly.
#![warn(missing_docs)]
#![forbid(unsafe_code)]
use crossbeam_channel::Sender;
use log::LevelFilter;
use std::env;
use std::path::Path;
use std::sync::Once;

/// Debugging macro to print the given expression and its value, with the module, function and line number
#[macro_export]
macro_rules! watch {
    ( $( $x:expr ),* ) => {
        {
            $(
               ::log::trace!("{}: {}", stringify!($x), format!("{:#?}", $x).replace("\\n", "\n"));
            )*
        }
    };
}

/// Either send a [XvcOutputLine::Error] value to the given channel, or log via `log` crate
#[macro_export]
macro_rules! error {
    ($fmt:literal $(, $x:expr )* ) => {
        {
            ::log::error!($fmt $(,$x)*);
        }
    };
    ( $channel:expr, $fmt:literal $(, $x:expr )* ) => {
        {
            (&$channel).send(Some(::xvc_logging::XvcOutputLine::Error(format!($fmt $(, $x)*)))).unwrap();
        }
    };
}

/// Either send [XvcOutputLine::Info] to the given channel, or log via `log` crate
#[macro_export]
macro_rules! info {
    ($fmt:literal $(, $x:expr )* ) => {
        {
            ::log::info!($fmt $(, $x)*);
        }
    };
    ( $channel:expr, $fmt:literal $(, $x:expr )* ) => {
        {
            (&$channel).send(Some(::xvc_logging::XvcOutputLine::Info(format!($fmt $(,$x)*)))).unwrap();
        }
    };
}

/// Either send [XvcOutputLine::Warn] to the given channel, or log via `log` crate
#[macro_export]
macro_rules! warn {
    ($fmt:literal $(, $x:expr )* ) => {
        {
            ::log::warn!($fmt $(, $x)*);
        }
    };
    ( $channel:expr, $fmt:literal $(, $x:expr )* ) => {
        {
            (&$channel).send(
                Some(
                    ::xvc_logging::XvcOutputLine::Warn(
                        format!($fmt $(, $x)*)))).unwrap();
        }
    };
}

/// Either send [XvcOutputLine::Debug] to the given channel, or log via `log` crate
#[macro_export]
macro_rules! debug {
    ($fmt:literal $(, $x:expr )* ) => {
        {
            ::log::debug!($fmt $(, $x)*);
        }
    };
    ( $channel:expr, $fmt:literal $(, $x:expr )* ) => {
        {
                    (&$channel).send(Some(::xvc_logging::XvcOutputLine::Debug(format!($fmt $(, $x)*)))).unwrap();
        }
    };
}

/// Either send [XvcOutputLine::Trace] to the given channel, or log via `log` crate
#[macro_export]
macro_rules! trace {
    ($fmt:literal $(, $x:expr )* ) => {
        {
            ::log::trace!($fmt $(, $x)*);
        }
    };
    ( $channel:expr, $fmt:literal $(, $x:expr ),* ) => {
        {
            (&$channel).send(Some(::xvc_logging::XvcOutputLine::Trace(format!("{} [{}::{}]", format!($fmt $(, $x)*), file!(), line!())))).unwrap();
        }
    };
}

/// Either send [XvcOutputLine::Output] to the given channel, or print to stdout
#[macro_export]
macro_rules! output {
    ($fmt:literal $(, $x:expr )* ) => {
        {
            ::std::println!($fmt $(, $x)*);
        }
    };
    ( $channel:expr, $fmt:literal $(, $x:expr )* ) => {
        {
            (&$channel).send(Some(::xvc_logging::XvcOutputLine::Output(format!($fmt $(, $x)*)))).unwrap();
        }
    };
}

/// Either send [XvcOutputLine::Panic] to the given channel, or print to stdout
#[macro_export]
macro_rules! panic {
    ($fmt:literal $(, $x:expr )* ) => {
        {
            watch!($fmt $(, $x)*);
            ::std::panic!($fmt $(, $x)*);
        }
    };
    ( $channel:expr, $fmt:literal $(, $x:expr )* ) => {
        {
            (&$channel).send(Some(::xvc_logging::XvcOutputLine::Panic(format!("{} [{}::{}]",
                                                                 format!($fmt $(, $x)*), file!(), line!())))).unwrap();
            ::std::panic!($fmt $(, $x)*);
        }
    };
}

/// Either send [XvcOutputLine::Tick] to the given channel, or print dots to stdout
#[macro_export]
macro_rules! tick {
    ( $channel:ident, $n:literal) => {{
        (&$channel)
            .send(Some(::xvc_logging::XvcOutputLine::Tick($n)))
            .unwrap();
    }};
    ($n:literal) => {{
        for _ in 0..$n {
            ::std::print!(".");
        }
    }};
}

/// Unwrap the result of an expression, and if it is an error, send it to the given channel
/// and panic.
/// This is mostly to be used in `for_each` blocks, where the error is not propagated.
#[macro_export]
macro_rules! uwr {
    ( $e:expr, $channel:expr ) => {{
        match $e {
            Ok(v) => v,
            Err(e) => {
                (&$channel)
                    .send(Some(::xvc_logging::XvcOutputLine::Panic(format!(
                        "{:?}, [{}::{}]",
                        e,
                        file!(),
                        line!()
                    ))))
                    .unwrap();
                ::std::panic!("{:?}", e);
            }
        }
    }};
}

/// Unwrap an option, and if it is an error, send it to the given channel
/// and panic.
/// This is mostly to be used in `for_each` blocks, where the error is not propagated.
#[macro_export]
macro_rules! uwo {
    ( $e:expr, $channel:expr ) => {{
        match $e {
            Some(v) => v,
            None => {
                watch!($e);
                let msg = format!(
                    "None from the expression: {} [{}::{}]",
                    stringify!($e),
                    file!(),
                    line!()
                );
                (&$channel)
                    .send(Some(::xvc_logging::XvcOutputLine::Panic(msg.clone())))
                    .unwrap();
                ::std::panic!("{}", msg);
            }
        }
    }};
}

/// Logging Initializer
static INIT: Once = Once::new();

/// Init logging if it's not initialized before.
/// Uses [Once] to run (non-public fn) `init_logging` once.
pub fn setup_logging(term_level: Option<LevelFilter>, file_level: Option<LevelFilter>) {
    INIT.call_once(|| init_logging(term_level, file_level));
}

fn init_logging(term_level: Option<LevelFilter>, file_level: Option<LevelFilter>) {
    let logfilename = &format!("{}/xvc.log", env::temp_dir().to_string_lossy(),);

    let logfile = Path::new(&logfilename);

    let mut dispatch = fern::Dispatch::new().format(|out, message, record| {
        out.finish(format_args!(
            "[{}][{}::{}] {}",
            record.level(),
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
                debug!("Terminal logger enabled with level: {:?}", level);
            };
            if let Some(level) = file_level {
                debug!(
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

/// Different channels of outputs Xvc can print
#[derive(Clone, Debug)]
pub enum XvcOutputLine {
    /// The output that we should be reporting to user
    Output(String),
    /// For informational messages
    Info(String),
    /// For debug output to show the internals of Xvc
    Debug(String),
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

/// The channel type to send and receive output/log/debug messages
pub type XvcOutputSender = Sender<Option<XvcOutputLine>>;

impl XvcOutputLine {
    /// print [INFO] `s`
    pub fn info(s: &str) -> Self {
        Self::Info(s.to_string())
    }
    /// print [DEBUG]
    pub fn debug(s: &str) -> Self {
        Self::Debug(s.to_string())
    }
    /// print [WARN] `s`
    pub fn warn(s: &str) -> Self {
        Self::Warn(s.to_string())
    }

    /// print [ERROR] `s`
    pub fn error(s: &str) -> Self {
        Self::Error(s.to_string())
    }

    /// print [PANIC] `s`
    ///
    /// Does not panic. Developer should call `panic!` macro separately.
    pub fn panic(s: &str) -> Self {
        Self::Panic(s.to_string())
    }

    /// Increment in progress bar
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
