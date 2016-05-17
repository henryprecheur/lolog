#![crate_type = "lib"]
#![crate_name = "lolog"]

extern crate log;

use std::io;
use std::iter::FromIterator;

use log::{LogRecord, LogLevel, LogMetadata};
use std::sync::{Arc, Mutex};

pub struct Logger<'a> {
    max_level: LogLevel,
    outputs: Arc<Mutex<Vec<&'a mut (io::Write + Sync + Send)>>>,
}

impl<'a> log::Log for Logger<'a> {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= self.max_level
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            let o = self.outputs.clone();
            let mut lg = o.lock().unwrap();

            for i in lg.iter_mut() {
                let err = i.write_all(
                    format!("{} - {}", record.level(), record.args()).as_str().as_bytes());
                match err {
                    Ok(_) => {},
                    Err(e) => { panic!(e) },
                }
            }
        }
    }
}

impl<'a> Logger<'a> {
    pub fn init<'b, I: Iterator<Item=&'b mut (io::Write + Sync + Send)>>
    (max_level: LogLevel, files: I) {
        let outputs = Vec::from_iter(files);

        Logger{
            max_level: max_level,
            outputs: Arc::new(Mutex::new(outputs)),
        };
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
    }
}
