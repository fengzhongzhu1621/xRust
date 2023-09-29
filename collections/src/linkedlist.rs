use std::collections::LinkedList;
use std::sync::{Arc, Mutex};

pub fn common_thread_safe_linkedlist() {
    let list1: LinkedList<u32> = LinkedList::new();
    let list2 = Arc::new(Mutex::new(list1));

    let mut handles = vec![];
    for i in 0..10 {
        let list3 = Arc::clone(&list2);
        handles.push(std::thread::spawn(move || {
            let mut v = list3.lock().unwrap();
            v.push_back(i);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("LinkedList: {:?}", list2.lock().unwrap());
}
