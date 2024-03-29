use asyncwait::*;

fn main() {
    // 使用 futures_lite 执行一步任务
    stream();
    futures_lite_io();

    // 模拟 future 运行
    timefuture_async();

    // tokio 运行时
    tokio_async();

    // futures::executor 运行时
    futures_async();

    // 其它运行时
    futures_lite_async();
    async_std_demo();
    smol_async();

    // 等待task执行完
    join();

    // 执行select操作
    select();
    futures_select();

    // 异步 trait
    async_trait_example();

    // async_lock
    async_lock_mutex();
    async_lock_rwlock();
    async_lock_barrier();
    async_lock_semaphore();

    // awaitgroup
    awaitgroup_example();

    // triggered
    triggered_example();

    // barrage
    barrage_example();

    // sema
    async_weighted_semaphore_example();

    // singleflight
    singleflight_example();
    async_singleflight_example();
}
