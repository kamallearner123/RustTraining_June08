

use std::sync::{Arc,Mutex};
use std::thread;

fn worker(id:i8, ref_data: Arc<Mutex<Vec<i8>>>) -> Option<i8> {
    println!("Running thread!!! {}, data = {:?}\n", id, ref_data);
    {
        let mut data = ref_data.lock().unwrap();
        data.push(id*10);
    }
    return Some(id*10);
}

// & :-> let r1 = &data; let r2 = &data;
// Rc : Reference Count - Single Thread
// Arc: Atomic variable - Multiple Thread


fn main() {
    let data = Arc::new(Mutex::new(vec![1,2,3]));
    let mut handles = Vec::new();


    // Module : mpsc

    let (tx, rx) = mpsc::channel();

    // Creating Rc (single thread) -> Arc (Multiple thread)
    // Arc -> Write (need to Mutex lock)
    // Rc: Multiple references to the same content
    // RcCell: Get mutable reference and modify
    for i in 0..5 {

        let ref_data = Arc::clone(&data); // Creating new reference 
        
        let handle = thread::spawn( move || {
            worker(i, ref_data);
        });
        handles.push(handle);
    }

    for handle in handles {
        let result = handle.join().unwrap();
        println!("result {:?}", result);
    }

    println!("data = {:?}", data.lock().unwrap());
}
    
