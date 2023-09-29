use std::sync::{Arc, Mutex};


pub fn common_thread_safe_vec() {
    let vec1 = vec![];
    let vec2 = Arc::new(Mutex::new(vec1));

    let mut handles = vec![];
    for i in 0..10 {
        let vec3 = Arc::clone(&vec2);
        handles.push(std::thread::spawn(move || {
            let mut v = vec3.lock().unwrap();
            v.push(i);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("vec: {:?}", vec2.lock().unwrap());
}