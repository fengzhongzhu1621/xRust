// 导入thread根模块，即lib.rs定义的模块
use thread::*;

fn main() {
    start_one_thread();
    start_one_thread_result();
    start_two_threads();
    start_n_threads();

    current_thread();

    start_thread_with_sleep();
    start_thread_with_yield_now();

    // start_thread_with_priority();
    // thread_builder();

    // 测试线程输入参数所有权的转移
    start_one_thread_with_move();
    // start_one_thread_with_move2

    // 测试线程局部变量
    start_threads_with_threadlocal();

    // 测试暂停和唤醒线程
    thread_park();
    thread_park2();
    thread_park_timeout();
    park_thread();

    // 测试作用域线程
    start_scoped_threads();
    crossbeam_scope();
    rayon_scope();

    // 测试channel发送非send数据
    send_wrapper();

    // 测试线程总量
    print_thread_amount();

    // 控制线程的执行
    control_thread();

    #[cfg(not(target_os = "macos"))]
    use_affinity();

    // 使用宏来创建子线程
    go_thread();

    // 获得线程和CPU的数量
    info();

    // 模拟线程panic
    panic_example();
    panic_caught_example()
}
