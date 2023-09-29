use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use dashmap::DashMap;
use std::thread;
use flurry;
use evmap;


pub fn common_thread_safe_collections() {
    let map: HashMap<i32, i32> = HashMap::new();
    let m = Arc::new(Mutex::new(map));

    // 使用线程给字典添加数据
    let mut handles = vec![];
    for i in 0..10 {
        let m = Arc::clone(&m);
        handles.push(std::thread::spawn(move || {
            let mut map = m.lock().unwrap();
            map.insert(i, i);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("HashMap: {:?}", *m.lock().unwrap());
}

pub fn hashmap_example() {
    let map = Arc::new(DashMap::new());

    let map1 = map.clone();
    let whandle = thread::spawn(move || {
        map1.insert(1, 2);
        map1.insert(2, 3);
    });

    let map2 = map.clone();
    let rhandle = thread::spawn(move || {
        loop {
            if let Some(v) = map2.get(&1) {
                println!("get value {} for key 1", *v);
                break;
            }
        }

        loop {
            if let Some(v) = map2.get(&2) {
                println!("get value {} for key 2", *v);
                break;
            }
        }
    });

    whandle.join().unwrap();
    rhandle.join().unwrap();
}

pub fn flurry_hashmap() {
    let map = flurry::HashMap::new();

    assert_eq!(map.pin().insert(37, "a"), None);
    assert_eq!(map.pin().is_empty(), false);
}

pub fn evmap_example() {
    let (book_reviews_r, mut book_reviews_w) = evmap::new();
    
    let readers: Vec<_> = (0..4)
        .map(|_| {
            let r = book_reviews_r.clone();
            thread::spawn(move || {
                loop {
                    let l = r.len();
                    if l == 0 {
                        thread::yield_now();
                    } else {
                        // the reader will either see all the reviews,
                        // or none of them, since refresh() is atomic.
                        assert_eq!(l, 4);
                        break;
                    }
                }
            })
        })
        .collect();

    // do some writes
    book_reviews_w.insert("Adventures of Huckleberry Finn", "My favorite book.");
    book_reviews_w.insert("Grimms' Fairy Tales", "Masterpiece.");
    book_reviews_w.insert("Pride and Prejudice", "Very enjoyable.");
    book_reviews_w.insert("The Adventures of Sherlock Holmes", "Eye lyked it alot.");
    // expose the writes
    book_reviews_w.refresh();

    // you can read through the write handle
    assert_eq!(book_reviews_w.len(), 4);

    // the original read handle still works too
    assert_eq!(book_reviews_r.len(), 4);

    // all the threads should eventually see .len() == 4
    for r in readers.into_iter() {
        assert!(r.join().is_ok());
    }
}
