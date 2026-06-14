use std::time::Instant;

// Simple fast PRNG so we don't need external crates
struct XorShift {
    state: u32,
}
impl XorShift {
    fn new(seed: u32) -> Self { Self { state: seed } }
    fn next(&mut self) -> i32 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.state = x;
        x as i32
    }
}

fn main() {
    let num_elements = 50_000_000;
    println!("Generating {} random integers...", num_elements);

    let mut data = Vec::with_capacity(num_elements);
    let mut rng = XorShift::new(42);
    
    for _ in 0..num_elements {
        data.push(rng.next());
    }

    println!("Starting Rust slice::sort_unstable...");

    let start_time = Instant::now();
    
    data.sort_unstable();
    
    let duration = start_time.elapsed();

    println!("Sorting complete!");
    println!("Rust Sorting Time: {:.4} seconds", duration.as_secs_f64());
    
    // Print something to prevent over-optimization
    print!("First 5 elements: ");
    for i in 0..5 { print!("{} ", data[i]); }
    print!("\nLast 5 elements: ");
    for i in num_elements-5..num_elements { print!("{} ", data[i]); }
    println!();
}
