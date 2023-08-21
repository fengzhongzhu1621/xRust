#![allow(unused_variables)]
#![allow(unused_assignments)]

use std::sync::Once;

pub fn once_example() {
    // 同步原语，可用于运行一次性初始化。 对于 FFI 或相关功能的一次性初始化很有用。
    // 该类型只能用 Once::new() 构造。
    let once = Once::new();
    let mut val: usize = 0;

    once.call_once(|| {
        val = 100;
    });

    if once.is_completed() {
        println!("Once: {}", val);
    }
}
