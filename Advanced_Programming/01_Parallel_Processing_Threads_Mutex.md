# Parallel Processing using Threads and Mutex

In Rust, safe concurrency is one of the language's main goals. We can achieve parallel processing by spawning threads and safely sharing data using a `Mutex` (Mutual Exclusion).

![Parallel Processing with Threads and Mutex](/home/kamal/.gemini/antigravity/brain/501820e8-442d-4db7-86dc-3148d95583d4/parallel_processing_1783348045413.png)

## Example: Sharing State Across Threads safely

Here's an example that demonstrates spawning multiple threads and modifying a shared counter. We use `Arc` (Atomic Reference Counted) to share ownership of the `Mutex` across multiple threads.

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // We wrap our data (an integer `0`) in a Mutex to ensure mutually exclusive access.
    // Then we wrap the Mutex in an Arc to allow multiple threads to own a reference to it.
    let counter = Arc::new(Mutex::new(0));
    
    // A vector to hold the join handles of the spawned threads.
    let mut handles = vec![];

    for _ in 0..10 {
        // Clone the Arc to give a reference to the specific thread.
        let counter_clone = Arc::clone(&counter);
        
        // Spawn a new thread.
        let handle = thread::spawn(move || {
            // Lock the mutex to access the data. 
            // `lock()` blocks the current thread until it can acquire the lock.
            // `unwrap()` is used because locking might fail if another thread panicked while holding the lock.
            let mut num = counter_clone.lock().unwrap();
            
            // Dereference the lock guard and modify the underlying data.
            *num += 1;
            
            // The lock is automatically released when `num` goes out of scope here.
        });
        
        handles.push(handle);
    }

    // Wait for all threads to complete.
    for handle in handles {
        handle.join().unwrap();
    }

    // Print the final value. It should be 10.
    println!("Final Counter Result: {}", *counter.lock().unwrap());
}
```
