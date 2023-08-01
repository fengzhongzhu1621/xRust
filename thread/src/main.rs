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
}
