use atomic_float;
use atomicbox::AtomicBox;
use atomig::Atomic;
use portable_atomic::{AtomicF32, AtomicF64, AtomicI128, AtomicU128};
use std::sync::atomic::Ordering::Relaxed;
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

pub fn portable_atomic_i128() {
    let mut some_var = AtomicI128::new(10);
    assert_eq!(*some_var.get_mut(), 10);

    // 修改值
    *some_var.get_mut() = 5;
    assert_eq!(some_var.load(Ordering::SeqCst), 5);

    assert_eq!(some_var.load(Ordering::Relaxed), 5);
}

pub fn portable_atomic_u128() {
    let mut some_var = AtomicU128::new(10);
    assert_eq!(*some_var.get_mut(), 10);
    *some_var.get_mut() = 5;
    assert_eq!(some_var.load(Ordering::SeqCst), 5);

    assert_eq!(some_var.load(Ordering::Relaxed), 5);
}

pub fn portable_atomic_f32() {
    let mut some_var = AtomicF32::new(10.0);
    assert_eq!(*some_var.get_mut(), 10.0);
    *some_var.get_mut() = 5.0;
    assert_eq!(some_var.load(Ordering::SeqCst), 5.0);

    assert_eq!(some_var.load(Ordering::Relaxed), 5.0);
}

pub fn portable_atomic_f64() {
    let mut some_var = AtomicF64::new(10.0f64);
    assert_eq!(*some_var.get_mut(), 10.0);
    *some_var.get_mut() = 5.0;
    assert_eq!(some_var.load(Ordering::SeqCst), 5.0);

    assert_eq!(some_var.load(Ordering::Relaxed), 5.0);
}

pub fn atomic_float_example() {
    let some_var = atomic_float::AtomicF32::new(800.0f32);
    some_var.fetch_add(30.0, Relaxed);
    some_var.fetch_sub(-55.0, Relaxed);
    some_var.fetch_neg(Relaxed);

    assert_eq!(some_var.load(Relaxed), -885.0);

    let some_var = atomic_float::AtomicF64::new(800.0f64);
    some_var.fetch_add(30.0, Relaxed);
    some_var.fetch_sub(-55.0, Relaxed);
    some_var.fetch_neg(Relaxed);

    assert_eq!(some_var.load(Relaxed), -885.0);
}

pub fn atomig_example() {
    let some_var = Atomic::new(0);
    some_var.store(800, Relaxed);

    some_var.fetch_add(30, Relaxed);
    some_var.fetch_sub(-55, Relaxed);

    assert_eq!(some_var.load(Relaxed), 885);
}

pub fn atomicbox_examples() {
    let atom = AtomicBox::new(Box::new("one"));
    let mut boxed = Box::new("two");
    atom.swap_mut(&mut boxed, Ordering::AcqRel);
    assert_eq!(*boxed, "one");
}
