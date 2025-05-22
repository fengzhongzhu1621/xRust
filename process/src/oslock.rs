
use std::time::{Duration, Instant};
use process_lock::ProcessLock;


/// 尝试获取一个进程锁
pub fn process_lock() {
    // 创建一个新的进程锁实例，锁的标识符是".process_lock"(文件名)
    let lock = ProcessLock::new(String::from(".process_lock"), None);
    // 记录当前时间作为开始时间
    let start = Instant::now();

    loop {
        // 判断锁是否获取成功
        if lock.is_ok() {
            println!("lock success");
            break;
        }

        // 超时检查
        if start.elapsed() > Duration::from_millis(500) {
            println!("lock timeout");
            break;
        }

        // 循环结束后再等待500毫秒
        std::thread::sleep(Duration::from_millis(100));
    }

    std::thread::sleep(Duration::from_millis(500));

}