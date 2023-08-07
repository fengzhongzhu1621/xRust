use asyncwait::*;

fn main() {
    // 使用 futures_lite 执行一步任务
    stream();
    futures_lite_io();

    // 模拟 future 运行
    timefuture_async()
}
