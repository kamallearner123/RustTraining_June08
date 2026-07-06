# Handling Huge Datasets using Multiple Threads

When processing massive amounts of data (like logs, large files, or extensive mathematical computations), a single thread can be a bottleneck. By partitioning the data and distributing the workload across multiple threads, we can drastically reduce processing time. The `rayon` crate is the de facto standard for data parallelism in Rust.

![Huge Dataset Processing](/home/kamal/.gemini/antigravity/brain/501820e8-442d-4db7-86dc-3148d95583d4/huge_dataset_threads_1783348082747.png)

## Example: Processing a Large Array with Rayon

`rayon` makes it incredibly easy to convert sequential iterators into parallel iterators. It handles thread pooling and work-stealing under the hood, ensuring optimal CPU utilization.

```rust
// Add `rayon = "1.8"` to your Cargo.toml dependencies to use this example.
use rayon::prelude::*;
use std::time::Instant;

fn main() {
    // Create a huge dataset: a vector with 10 million elements.
    let dataset: Vec<u64> = (1..=10_000_000).collect();

    println!("Dataset generated with {} elements.", dataset.len());

    let start_time = Instant::now();

    // Instead of using the sequential `.iter()`, we use `.par_iter()` provided by Rayon.
    // This automatically splits the vector into chunks and processes them on multiple threads.
    let sum_of_squares: u64 = dataset.par_iter()
        .map(|&num| {
            // Perform a computationally intensive operation (simulated here with multiplication).
            num * num
        })
        .sum(); // `sum()` is also parallelized and aggregates the results from all threads.

    let duration = start_time.elapsed();

    println!("Parallel processing completed in: {:?}", duration);
    // Print the result (using wrapping multiplication to avoid overflow panics in debug mode if needed, 
    // but u64 is large enough for this example conceptually).
    println!("Sum of squares: {}", sum_of_squares);
}
```
