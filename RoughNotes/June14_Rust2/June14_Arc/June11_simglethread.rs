

use std::sync::mpsc::{self, Sender};
use std::thread;

fn worker(id:i8, tx: Sender<i32>) -> i8 {
    tx.send(100).unwrap();
    return 0;
}

fn main() {
    let (tx, rx) = mpsc::channel();

        
    let handle = thread::spawn( move || {
            worker(1, tx);
    });
    
    let result = rx.recv().unwrap();
    println!("result = {}", result);
}
    
