use log::{Record, Level, Metadata};

pub struct Log;

impl log::Log for Log {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{:?} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}