use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

pub fn arc_example() {
    // 当线程之间所有权需要共享时，可以使用Arc（共享引用计数，Atomic Reference Counted 缩写）可以使用。
    // 这个结构通过 Clone 实现可以为内存堆中的值的位置创建一个引用指针，同时增加引用计数器。
    // 由于它在线程之间共享所有权，因此当指向某个值的最后一个引用指针退出作用域时，该变量将被删除。
    let five = Arc::new(5);

    for _ in 0..10 {
        let five = Arc::clone(&five);

        thread::spawn(move || {
            println!("arc_example {five:?}");
        });
    }
}

pub fn arc_example2() {
    let val = Arc::new(AtomicUsize::new(5));

    for _ in 0..10 {
        let val = Arc::clone(&val);

        thread::spawn(move || {
            let v = val.fetch_add(1, Ordering::SeqCst);
            println!("arc_example2 {v:?}");
        });
    }
}
