use std::env;
use std::process;

use log::*;

use crazcalm_logger::{LevelFilter, SimpleFormatter, StderrLogger};

#[test]
fn test_stderr_logger() {
    let expected_log = r"File: tests\stderr_logger_test.rs, Level: INFO,";

    if env::var("YOU_ARE_TESTING_NOW").is_ok() {
        // Init from the env (which should set the max level to `Debug`)
        let _ = StderrLogger::init(LevelFilter::Debug, SimpleFormatter::new()).unwrap();

        info!("What are you doing");
        return;
    }

    let exe = env::current_exe().unwrap();
    let out = process::Command::new(exe)
        .env("YOU_ARE_TESTING_NOW", "1")
        .env("RUST_LOG", "debug")
        .output()
        .unwrap_or_else(|e| panic!("Unable to start child process: {e}"));
    if out.status.success() {
        let result = str::from_utf8(&out.stderr).unwrap();
        println!("result -> {}", result);
        assert!(result.to_string().contains(expected_log));
        return;
    }

    println!("test failed: {}", out.status);
    println!("--- stdout\n{}", str::from_utf8(&out.stdout).unwrap());
    println!("--- stderr\n{}", str::from_utf8(&out.stderr).unwrap());
    process::exit(1);
}
