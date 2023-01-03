use log::LevelFilter;
use log::{error, info};
use std::env;
use std::path::Path;
use std::sync::Once;

// Logging

static INIT: Once = Once::new();

pub fn setup_logging(term_level: Option<LevelFilter>, file_level: Option<LevelFilter>) {
    INIT.call_once(|| init_logging(term_level, file_level));
}

fn init_logging(term_level: Option<LevelFilter>, file_level: Option<LevelFilter>) {
    let logfilename = &format!(
        "{}/xvc-{}.log",
        env::temp_dir().to_string_lossy(),
        chrono::Local::now().format("%Y%m%d-%H%M%S")
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
