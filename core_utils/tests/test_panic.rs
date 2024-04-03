use core_utils::panic::register_panic_hook;
use env_logger;
use log::info;
use log::LevelFilter;

#[test]
fn test_panic_set_hook() {
    let _ = env_logger::builder()
        .is_test(true)
        .filter(None, LevelFilter::Debug)
        .try_init();

    register_panic_hook();
}
