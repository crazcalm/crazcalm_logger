use log::Record;

pub mod simple_formatter;

pub use simple_formatter::SimpleFormatter;

pub trait LogFormatter: Send + Sync + 'static {
    fn format(&self, record: &Record<'_>) -> String;
}
