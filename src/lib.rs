#![crate_type = "lib"]
#![crate_name = "lolog"]

///
/// Work in progress, don't use it.
///
///     lolog::init(log::LogLevel::Info, std::io::stderr());
///
extern crate log;
extern crate time;

use std::io;

use log::{LogRecord, LogLevel, LogMetadata, SetLoggerError};
use std::sync::{Arc, Mutex};

type Formatter = Fn(&LogRecord) -> String + Send + Sync;

pub struct Logger<Writer: io::Write + Send> {
    max_level: LogLevel,
    output: Arc<Mutex<Writer>>,
    formatter: Box<Formatter>,
}

impl<W: io::Write + Send> log::Log for Logger<W> {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= self.max_level
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            let o = self.output.clone();
            let mut f = o.lock().unwrap();

            // let line = format!("{} {}", time::now().rfc3339(), record.args());
            let line = (self.formatter)(record);
            f.write_all(line.as_bytes())
             .expect("Couldn't write to log file");
        }
    }
}

pub fn default_formatter(record: &LogRecord) -> String {
    match record.target() {
        "" => {
            format!("{} {} {}",
                    time::now().rfc3339(),
                    record.level(),
                    record.args())
        }
        _ => {
            format!("{} {} {} {}",
                    time::now().rfc3339(),
                    record.target(),
                    record.level(),
                    record.args())
        }
    }
}

impl<W: io::Write + Send> Logger<W> {
    pub fn new(max_level: LogLevel, writer: W) -> Self {
        Logger {
            max_level: max_level,
            output: Arc::new(Mutex::new(writer)),
            formatter: Box::new(default_formatter),
        }
    }
}

pub fn install<W: 'static + io::Write + Send>(logger: Logger<W>) -> Result<(), SetLoggerError> {
    log::set_logger(move |max_log_level| {
        max_log_level.set(logger.max_level.to_log_level_filter());
        Box::new(logger)
    })
}

pub fn init<W: 'static + io::Write + Send>(level: LogLevel,
                                           output: W)
                                           -> Result<(), SetLoggerError> {
    log::set_logger(move |max_log_level| {
        max_log_level.set(level.to_log_level_filter());
        Box::new(Logger::new(level, output))
    })
}

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {}
}
