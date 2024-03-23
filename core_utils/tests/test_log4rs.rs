use core_utils::logger::log4rs::init_log;
use log::debug;
use log::LevelFilter;
use std::env;

#[test]
fn test_init_log() {
    let log_path = env::current_dir().unwrap().join("tests/test.log");
    init_log(log_path.as_path());

    debug!("test_init_log")
}
