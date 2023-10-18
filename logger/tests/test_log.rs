use log::Level::Debug;
use log::{debug, error, info, log, log_enabled, trace, warn, Level};
use std::module_path;

fn add(a: i32, b: i32) -> i32 {
    trace!(target: "call_function", "add");
    info!("{} + {}", a, b);

    a + b
}

#[test]
fn test_log_lever_fn() {
    let data = (1, "one");
    log!(Level::Error, "Received data: {}, {}", data.0, data.1);

    let private_data = "private";
    log!(target: "app_events", Level::Warn, "App warning:{}, {}, {}", data.0, data.1, private_data);

    let (err_info, port) = ("No connection", 22);
    error!("Error: {} on port {}", err_info, port);
    warn!("Waring: {} on port {}", err_info, port);
    info!("Info: {} on port {}", err_info, port);
    debug!("Debug: {} on port {}", err_info, port);
    trace!("Trace: {} on port {}", err_info, port);
}

#[test]
fn test_target() {
    add(1, 2);
}

/// 日志宏的第一个参数使用target:来定义标签名称，
/// 则会将标签名称缺省为调用这个宏的模块的路径
#[test]
fn test_default_target() {
    fn add(a: i32, b: i32) -> i32 {
        trace!(target: "call_function", "add");
        info!(target: module_path!(), "{} + {}", a, b);

        a + b
    }
    add(1, 2);
}

#[test]
fn test_log_enabled() {
    struct Point {
        x: usize,
        y: usize,
    }

    fn expensive_call() -> Point {
        Point { x: 1, y: 2 }
    }

    // 判断能否记录 Debug 消息
    if log_enabled!(Debug) {
        let data = expensive_call();
        // 耗时日志记录前进行判断
        debug!("expensive debug data: {} {}", data.x, data.y);
    }
    if log_enabled!(target: "Global", Debug) {
        let data = expensive_call();
        debug!(target: "Global", "expensive debug data: {} {}", data.x, data.y);
    }
}
