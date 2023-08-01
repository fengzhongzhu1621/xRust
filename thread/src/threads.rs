use std::thread;
use std::time::Duration;

pub fn start_one_thread() {
    // 该函数可以返回程序应使用的默认并行度的估计值，即它可以同时执行的计算数量的限制。
    // 也会考虑CPU 配额。例如，在一个有 8 个虚拟 CPU 但配额只允许 50% 使用率的容器中，
    // available_parallelism将返回 4。
    let count = thread::available_parallelism().unwrap().get();
    println!("available_parallelism: {}", count);

    // 创建一个子线程
    let handle = thread::spawn(|| {
        println!("Hello from a thread!");
    });

    // 等待子线程结束
    handle.join().unwrap();
}

pub fn start_one_thread_result() {
    // 创建一个子线程，并返回线程的执行结果
    let handler = thread::spawn(|| {
        println!("Hello from a thread!");
        200
    });

    match handler.join() {
        Ok(v) => println!("thread result is : {}", v),
        Err(e) => println!("error: {:?}", e),
    }
}

pub fn start_two_threads() {
    let handle1 = thread::spawn(|| {
        println!("Hello from a thread1!");
    });

    let handle2 = thread::spawn(|| {
        println!("Hello from a thread2!");
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}

pub fn start_n_threads() {
    // 定义线程的数量，有符号整数类型
    const N: isize = 10;

    // 从迭代器创建多个线程
    let handles: Vec<_> = (0..N)
        .map(|i| {
            thread::spawn(move || {
                println!("Hello from a thread{}!", i);
            })
        })
        .collect();

    // handles.into_iter().for_each(|h| h.join().unwrap());

    for handle in handles {
        handle.join().unwrap();
    }
}

pub fn current_thread() {
    // 获得主线程
    let current_thread = thread::current();
    println!(
        "current thread: {:?},{:?}",
        current_thread.id(),
        current_thread.name()
    );

    // 创建子线程
    let builder = thread::Builder::new()
        .name("foo".into()) // set thread name
        .stack_size(32 * 1024); // set stack size

    let handler = builder
        .spawn(|| {
            // 获得当前子线程
            let current_thread = thread::current();
            println!(
                "child thread: {:?},{:?}",
                current_thread.id(),
                current_thread.name()
            );
        })
        .unwrap();

    handler.join().unwrap();
}

pub fn start_thread_with_sleep() {
    let handle1 = thread::spawn(|| {
        thread::sleep(Duration::from_millis(2000));
        println!("Hello from a thread3!");
    });

    let handle2 = thread::spawn(|| {
        thread::sleep(Duration::from_millis(1000));
        println!("Hello from a thread4!");
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}

pub fn start_thread_with_yield_now() {
    let handle1 = thread::spawn(|| {
        // 立即放弃CPU，将线程交还给调度器，自己则进入就绪队列等待下一轮的调度
        thread::yield_now();
        println!("yield_now!");
    });

    let handle2 = thread::spawn(|| {
        // 立即放弃CPU，将线程交还给调度器，自己则进入就绪队列等待下一轮的调度
        thread::yield_now();
        println!("yield_now in another thread!");
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}
