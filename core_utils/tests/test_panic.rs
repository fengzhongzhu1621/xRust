use core_utils::panic::register_panic_hook;
use env_logger;
use log::LevelFilter;
use std::thread;
use std::time::Duration;

#[test]
fn test_panic_set_hook() {
    let _ = env_logger::builder()
        .is_test(true)
        .filter(None, LevelFilter::Debug)
        .try_init();

    register_panic_hook();

    thread::spawn(|| {
        panic!("child thread panic");
    });

    thread::sleep(Duration::from_millis(100));
}

#[test]
fn test_panic_unwrap() {
    let _ = env_logger::builder()
        .is_test(true)
        .filter(None, LevelFilter::Debug)
        .try_init();

    register_panic_hook();

    thread::spawn(|| {
        let _ = "abc".parse::<i32>().unwrap();
    });

    thread::sleep(Duration::from_millis(100));
}
