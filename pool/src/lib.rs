use fast_threadpool;
use rayon;
use scheduled_thread_pool;
use scoped_threadpool;
use smol::{io, net, prelude::*, Unblock};
use std::sync::mpsc::channel;
use std::time::Duration;
use threadpool::ThreadPool;

/// 计算斐波那契数列
fn fib(n: usize) -> usize {
    if n == 0 || n == 1 {
        return n;
    }
    let (a, b) = rayon::join(|| fib(n - 1), || fib(n - 2)); // runs inside of `pool`
    return a + b;
}

pub fn threadpool_example() {
    let n_workers = 4;
    let n_jobs = 8;
    // 创建线程池
    let pool = ThreadPool::new(n_workers);

    // 通过管道数据给主线程
    let (tx, rx) = channel();
    for _ in 0..n_jobs {
        let tx = tx.clone();
        pool.execute(move || {
            tx.send(1)
                .expect("channel will be there waiting for the pool");
        });
    }

    // 主线程获取子线程数据并计算
    let n = rx.iter().take(n_jobs).fold(0, |a, b| a + b);
    println!(" the result of threadpool_example is {}", n);
    assert_eq!(n, 8);
}

pub fn rusty_pool_example() {
    let pool = ThreadPool::new(4);

    for _ in 1..10 {
        pool.execute(|| {
            println!("Hello from a rusty_pool!");
        });
    }

    pool.join();
}

pub fn rayon_threadpool() {
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(8)
        .build()
        .unwrap();
    let n = pool.install(|| fib(20));
    println!("the result of rayon_threadpool is {}", n);
}

pub fn fast_threadpool_example() -> Result<(), fast_threadpool::ThreadPoolDisconnected> {
    let threadpool =
        fast_threadpool::ThreadPool::start(fast_threadpool::ThreadPoolConfig::default(), ())
            .into_sync_handler();

    let n = threadpool.execute(|_| 2 + 2)?;
    assert_eq!(4, n);

    println!("the result of fast_threadpool_example is {}", n);

    Ok(())
}

pub fn scoped_threadpool_example() {
    let mut pool = scoped_threadpool::Pool::new(4);

    let mut vec = vec![0, 1, 2, 3, 4, 5, 6, 7];

    // Use the threads as scoped threads that can
    // reference anything outside this closure
    pool.scoped(|s| {
        // Create references to each element in the vector ...
        for e in &mut vec {
            // ... and add 1 to it in a seperate thread
            s.execute(move || {
                *e += 1;
            });
        }
    });

    assert_eq!(vec, vec![1, 2, 3, 4, 5, 6, 7, 8]);
}

/// While a normal thread pool is only able to execute actions as soon as possible,
/// a scheduled thread pool can execute actions after a specific delay,
///  or execute actions periodically.
pub fn scheduled_thread_pool_example() {
    let (sender, receiver) = channel();

    let pool = scheduled_thread_pool::ScheduledThreadPool::new(4);
    let handle = pool.execute_after(Duration::from_millis(1000), move || {
        println!("Hello from a scheduled thread!");
        sender.send("done").unwrap();
    });

    let _ = handle;
    receiver.recv().unwrap();
}

/// 用于 Rust 的小型快速的异步运行时。
pub fn unblocking_smol() -> io::Result<()> {
    smol::block_on(async {
        // 发送http请求
        let mut stream = net::TcpStream::connect("example.com:80").await?;
        let req = b"GET / HTTP/1.1\r\nHost: example.com\r\nConnection: close\r\n\r\n";
        stream.write_all(req).await?;

        // 将http响应写入到标准输出
        let mut stdout = Unblock::new(std::io::stdout());
        io::copy(stream, &mut stdout).await?;
        Ok(())
    })
}
