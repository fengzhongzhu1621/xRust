use pool::*;

fn main() {
    // 线程池示例
    threadpool_example();
    rusty_pool_example();

    rayon_threadpool();

    fast_threadpool_example();

    scoped_threadpool_example();
    scheduled_thread_pool_example();

    // 异步编程
    unblocking_smol();
}
