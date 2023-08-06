pub mod threads;

// 将路径 ./threads.rs导入到模块
pub use threads::*;

/// 线程panic demo
pub fn panic_example() {
    println!("Hello, world!");

    // 创建一个子线程模拟panic
    let h = std::thread::spawn(|| {
        std::thread::sleep(std::time::Duration::from_millis(1000));
        panic!("boom");
    });

    let r = h.join();
    match r {
        Ok(r) => println!("All is well! {:?}", r),
        Err(e) => println!("Got an error! {:?}", e),
    }

    println!("Exiting main!")
}

pub fn panic_caught_example() {
    println!("Hello, panic_caught_example !");

    let h = std::thread::spawn(|| {
        std::thread::sleep(std::time::Duration::from_millis(1000));
        // 线程捕获panic
        let result = std::panic::catch_unwind(|| {
            panic!("boom");
        });

        println!("panic caught, result = {}", result.is_err());
    });

    let r = h.join();
    match r {
        Ok(r) => println!("All is well! {:?}", r),
        Err(e) => println!("Got an error! {:?}", e),
    }

    println!("Exiting main!")
}
