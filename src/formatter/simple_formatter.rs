use std::collections::HashMap;

use log::Record;

use crate::LogFormatter;

pub struct SimpleFormatter {}

impl SimpleFormatter {
    pub fn new() -> Self {
        Self {}
    }
}

impl LogFormatter for SimpleFormatter {
    fn format(&self, record: &Record<'_>) -> String {
        let mut dict = HashMap::new();
        let args = format_args!("{}", record.args());
        dict.insert("Message".to_string(), args.to_string());
        dict.insert("Level".to_string(), record.metadata().level().to_string());
        dict.insert("Target".to_string(), record.metadata().target().to_string());

        if record.module_path().is_some() {
            dict.insert(
                "Module_Path".to_string(),
                record.module_path().unwrap().to_string(),
            );
        }
        if record.file().is_some() {
            dict.insert("File".to_string(), record.file().unwrap().to_string());
        }
        if record.line().is_some() {
            dict.insert("Line".to_string(), record.line().unwrap().to_string());
        }

        let mut tuple_data: Vec<(String, String)> = dict
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();

        tuple_data.sort();

        let mut result: String = String::new();
        let result_len = tuple_data.len() - 1;
        for (index, (key, value)) in tuple_data.iter().enumerate() {
            result.push_str(key);
            result.push_str(": ");
            result.push_str(value);
            if index < result_len {
                result.push_str(", ");
            }
        }

        format!("{}\n", result)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    use log::Level;

    #[test]
    fn test_format() {
        let expected = "File: server.rs, Level: ERROR, Line: 144, Message: Error!, Module_Path: server, Target: myApp\n".to_string();

        let record = Record::builder()
            .args(format_args!("Error!"))
            .level(Level::Error)
            .target("myApp")
            .file(Some("server.rs"))
            .line(Some(144))
            .module_path(Some("server"))
            .build();

        let formatter = SimpleFormatter {};
        let result = formatter.format(&record);
        assert_eq!(expected, result);
    }
}
