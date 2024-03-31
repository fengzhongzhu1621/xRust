use core_utils::signal::ctrlc;
use std::sync::mpsc::channel;
use std::sync::{atomic::AtomicBool, OnceLock};

// 进程停止标识
static STOP_FLAG: OnceLock<AtomicBool> = OnceLock::new();

#[test]
fn test_ctrlc() {
    let (tx, rx) = channel();

    ctrlc::set_handler(move || {
        tx.send(()).expect("Could not send signal on channel.");
        match STOP_FLAG.set(AtomicBool::new(true)) {
            Ok(_) => {
                // 设置停止标记
                // ...
            }
            Err(_) => return,
        }
    })
    .expect("Error setting Ctrl-C handler");

    println!("Waiting for Ctrl-C...");
    // rx.recv().expect("Could not receive from channel.");

    if STOP_FLAG.get().is_some() {
        // 收到进程停止标记
        // ...
    }
    println!("Got it! Exiting...");
}
