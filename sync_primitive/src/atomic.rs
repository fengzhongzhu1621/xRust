use std::sync::atomic::{AtomicI32, AtomicUsize, Ordering};
use std::sync::Arc;
use std::{hint, thread};

pub fn atomic_example() {
    // 定义一个原子应用计数器指针
    let spinlock = Arc::new(AtomicUsize::new(1));

    // 创建智能指针的引用
    let spinlock_clone = Arc::clone(&spinlock);

    // 创建子线程修改指针的值
    let thread = thread::spawn(move || {
        spinlock_clone.store(0, Ordering::SeqCst);
    });

    // 等待该值被设置
    // Wait for the other thread to release the lock
    while spinlock.load(Ordering::SeqCst) != 0 {
        // 自旋等待
        hint::spin_loop();
    }

    if let Err(panic) = thread.join() {
        println!("Thread had an error: {panic:?}");
    } else {
        println!(
            "atomic_example atomic result: {}",
            spinlock.load(Ordering::SeqCst)
        );
    }
}

pub fn atomic_example2() {
    let v = AtomicI32::new(5);

    v.store(100, Ordering::SeqCst);

    println!("atomic_example2 atomic load:{}", v.load(Ordering::SeqCst));
    println!(
        "atomic_example2 atomic swap:{}",
        v.swap(5, Ordering::SeqCst)
    );
    println!(
        "atomic_example2 atomic swap:{}",
        v.compare_exchange(5, 100, Ordering::SeqCst, Ordering::SeqCst)
            .unwrap()
    );
    println!(
        "atomic_example2 atomic fetch_add:{}",
        v.fetch_add(1, Ordering::SeqCst)
    );
    println!("atomic_example2 atomic load:{}", v.load(Ordering::SeqCst));
    println!(
        "atomic_example2 atomic fetch_sub:{}",
        v.fetch_sub(1, Ordering::SeqCst)
    );
    println!("atomic_example2 atomic load:{}", v.load(Ordering::SeqCst));
    println!(
        "atomic_example2 atomic fetch_and:{}",
        v.fetch_and(1, Ordering::SeqCst)
    );
    println!("atomic_example2 atomic load:{}", v.load(Ordering::SeqCst));
    println!(
        "atomic_example2 atomic fetch_or:{}",
        v.fetch_or(1, Ordering::SeqCst)
    );
    println!("atomic_example2 atomic load:{}", v.load(Ordering::SeqCst));
    println!(
        "atomic_example2 atomic fetch_xor:{}",
        v.fetch_xor(1, Ordering::SeqCst)
    );
    println!("atomic_example2 atomic load:{}", v.load(Ordering::SeqCst));
    println!(
        "atomic_example2 atomic fetch_nand:{}",
        v.fetch_nand(1, Ordering::SeqCst)
    );
    println!("atomic_example2 atomic load:{}", v.load(Ordering::SeqCst));
}
