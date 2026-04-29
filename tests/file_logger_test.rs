use std::fs::{File, exists, read_to_string, remove_file};
use std::path::Path;

use crazcalm_logger::{FileLogger, LevelFilter, SimpleFormatter};

use log::*;

fn create_test_file<P: AsRef<Path>>(path: P) -> File {
    remove_test_file(&path);

    File::create(&path).unwrap()
}

fn remove_test_file<P: AsRef<Path>>(path: P) {
    if exists(&path).unwrap() {
        remove_file(&path).unwrap();
    }
}

#[test]
fn test_file_logger() {
    let expected_log = "File: tests\\file_logger_test.rs, Level: INFO, Line: 29, Message: What are you doing, Module_Path: file_logger_test, Target: file_logger_test\n".to_string();
    let file_name = "test_simple_logger.txt";
    let test_file = create_test_file(&file_name);

    let _ = FileLogger::init(LevelFilter::Debug, test_file, SimpleFormatter::new())
        .expect("Can write to destination");

    info!("What are you doing");

    let data = read_to_string(&file_name).unwrap();
    assert_eq!(data, expected_log);

    remove_test_file(&file_name);
}
