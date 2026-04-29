use std::io::{Write, stdout};

use log::{LevelFilter, Log, Metadata, Record, SetLoggerError, set_boxed_logger, set_max_level};

use crate::formatter::LogFormatter;

pub struct StdoutLogger<F: LogFormatter> {
    level: LevelFilter,
    formatter: F,
}

impl<F: LogFormatter> StdoutLogger<F> {
    pub fn new(log_level: LevelFilter, formatter: F) -> Box<StdoutLogger<F>> {
        Box::new(Self {
            level: log_level,
            formatter: formatter,
        })
    }

    pub fn init(log_level: LevelFilter, formatter: F) -> Result<(), SetLoggerError> {
        set_max_level(log_level);
        set_boxed_logger(Self::new(log_level, formatter))
    }
}

impl<F: LogFormatter> Log for StdoutLogger<F> {
    fn enabled(&self, metadata: &Metadata<'_>) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record<'_>) {
        if self.enabled(record.metadata()) {
            let output = self.formatter.format(record);

            stdout().write_all(output.as_bytes()).unwrap();
        }
    }

    fn flush(&self) {
        let _ = stdout().flush();
    }
}
