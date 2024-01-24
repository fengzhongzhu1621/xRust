use std::sync::mpsc;
use std::thread;

#[test]
fn test_close() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        for s in vec!["a", "b", "c"] {
            tx.send(s.to_string());
        }
        drop(tx);
    });

    for s in rx {
        println!("{}", s);
    }
}
