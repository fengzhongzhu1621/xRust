use std::sync::Arc;
use async_lock::{Mutex, RwLock, Barrier};
use std::thread;

pub fn async_lock_mutex() {
    let lock = Arc::new(Mutex::new(0));

    let lock1 = lock.clone();
    smol::block_on(async {
        let mut guard = lock1.lock().await;
        *guard += 1;
    });

    let lock2 = lock.clone();
    smol::block_on(async {
        let guard = lock2.lock().await;
        println!("lock2 {}", *guard);
    });
}

pub fn async_lock_rwlock() {
    let lock = Arc::new(RwLock::new(0));

    let lock1 = lock.clone();
    smol::block_on(async {
        let mut guard = lock1.write().await;
        *guard += 1;
    });

    let lock2 = lock.clone();
    smol::block_on(async {
        let guard = lock2.read().await;
        println!("lock2 {}", *guard);
    });
}

pub fn async_lock_barrier() {
    let barrier = Arc::new(Barrier::new(5));

    thread::scope(|s| {
        for _ in 0..5 {
            let barrier = barrier.clone();
            s.spawn(move || {
                smol::block_on(async {
                    println!("before wait");
                    barrier.wait().await;
                    println!("after wait");
                });
            });
        }
    });
}

pub fn async_lock_semaphore() {
    let s = Arc::new(async_lock::Semaphore::new(2));

    let _g1 = s.try_acquire_arc().unwrap();
    let g2 = s.try_acquire_arc().unwrap();

    assert!(s.try_acquire_arc().is_none());
    drop(g2);
    assert!(s.try_acquire_arc().is_some());
}