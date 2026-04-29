use std::io::Write;
use std::sync::Mutex;

use log::{LevelFilter, Log, Metadata, Record, SetLoggerError, set_boxed_logger, set_max_level};

use crate::formatter::LogFormatter;

pub struct FileLogger<D: Write + Send + Sync + 'static, F: LogFormatter> {
    level: LevelFilter,
    output_lock: Mutex<D>,
    formatter: F,
}

impl<D: Write + Send + Sync + 'static, F: LogFormatter> FileLogger<D, F> {
    pub fn new(log_level: LevelFilter, destination: D, formatter: F) -> Box<FileLogger<D, F>> {
        Box::new(Self {
            level: log_level,
            output_lock: Mutex::new(destination),
            formatter: formatter,
        })
    }

    pub fn init(
        log_level: LevelFilter,
        destination: D,
        formatter: F,
    ) -> Result<(), SetLoggerError> {
        set_max_level(log_level);
        set_boxed_logger(Self::new(log_level, destination, formatter))
    }
}

impl<D: Write + Send + Sync + 'static, F: LogFormatter> Log for FileLogger<D, F> {
    fn enabled(&self, metadata: &Metadata<'_>) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record<'_>) {
        if self.enabled(record.metadata()) {
            let mut destination = self.output_lock.lock().unwrap();

            let output = self.formatter.format(record);
            write!(*destination, "{}", output).unwrap()
        }
    }

    fn flush(&self) {
        let mut destination = self.output_lock.lock().unwrap();
        let _ = destination.flush();
    }
}
