use simple_mutex;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn mutex_example1() {
    let five = Arc::new(Mutex::new(5));

    let mut handles = vec![];

    for _ in 0..10 {
        let five = five.clone();

        let handle = thread::spawn(move || {
            let mut five = five.lock().unwrap();
            // 解引用，并计算
            *five += *five;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("mutex_example1 Mutex: {:?}", five.lock().unwrap());
}

pub fn mutex_example2_poison() {
    let lock = Arc::new(Mutex::new(0_u32));
    let lock2 = Arc::clone(&lock);

    let _ = thread::spawn(move || -> () {
        // This thread will acquire the mutex first, unwrapping the result of
        // `lock` because the lock has not been poisoned.
        let _guard = lock2.lock().unwrap();

        // This panic while holding the lock (`_guard` is in scope) will poison
        // the mutex.
        // 终止进程
        panic!();
    })
    .join();

    // 死锁
    // The lock is poisoned by this point, but the returned result can be
    // pattern matched on to return the underlying guard on both branches.
    let mut guard = match lock.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };

    *guard += 1;

    println!("mutex_example2_poison guard: {:?}", lock.lock().unwrap());
}

pub fn mutex_example3_drop() {
    let data_mutex = Arc::new(Mutex::new(vec![1, 2, 3, 4]));
    let res_mutex = Arc::new(Mutex::new(0));

    // 创建3个线程
    let mut threads = Vec::with_capacity(3);
    (0..3).for_each(|_| {
        let data_mutex_clone = Arc::clone(&data_mutex);
        let res_mutex_clone = Arc::clone(&res_mutex);

        threads.push(thread::spawn(move || {
            let mut data = data_mutex_clone.lock().unwrap();
            // This is the result of some important and long-ish work.
            let result = data.iter().fold(0, |acc, x| acc + x * 2);
            data.push(result);
            drop(data);

            *res_mutex_clone.lock().unwrap() += result;
        }));
    });

    let mut data = data_mutex.lock().unwrap();
    // This is the result of some important and long-ish work.
    let result = data.iter().fold(0, |acc, x| acc + x * 2);
    data.push(result);
    drop(data);
    *res_mutex.lock().unwrap() += result;

    // join thread
    threads.into_iter().for_each(|thread| {
        thread
            .join()
            .expect("The thread creating or execution failed !")
    });

    println!(
        "mutex_example3_drop Result: {:?}",
        res_mutex.lock().unwrap()
    );
}

pub fn simple_mutex_example() {
    let m = Arc::new(simple_mutex::Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let m = m.clone();
        handles.push(thread::spawn(move || {
            *m.lock() += 1;
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("m = {:?}", m);
}
