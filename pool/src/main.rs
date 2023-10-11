use pool::*;

fn main() {
    // 线程池示例
    threadpool_example();
    threadpool_example2();

    rusty_pool_example();
    rusty_pool_example2();
    rusty_pool_example3();

    rayon_threadpool();
    rayon_threadpool2();

    fast_threadpool_example();

    scoped_threadpool_examples();
    scheduled_thread_pool_examples();

    workerpool_rs_example();

    poolite_example();
    poolite_example2();

    executor_service_example();
    threadpool_executor_example();
    executors_example();

    // 异步编程
    unblocking_smol();
}
