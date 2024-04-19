use core_utils::panic::register_panic_hook;
use env_logger;
use human_panic::setup_panic;
use log::error;
use std::thread;
use std::time::Duration;

fn main() {
    env_logger::init();

    register_panic_hook();

    thread::spawn(|| {
        panic!("error");
        // let _ = "abc".parse::<i32>().unwrap();
    });
    thread::sleep(Duration::from_secs(1));
}
