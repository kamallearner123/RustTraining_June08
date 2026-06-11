# Performance Comparison: C++ vs Rust

This directory contains a performance comparison between a C++ application and a Rust application executing the exact same data processing task. 

## The Task
The applications are designed to perform a typical I/O-bound and CPU-bound task: parsing a massive CSV file and computing mathematical aggregations. 

1. Read `data.csv` (contains millions of rows of `id,value1,value2`).
2. Parse each row, skipping the `id`.
3. Extract `value1` and `value2` as floating-point numbers.
4. Compute the product (`value1 * value2`) and add it to a running total.
5. Print the final sum and elapsed time.

## Project Structure
- `generate_data.py`: A Python script that generates the `data.csv` file with 5,000,000 rows of random data.
- `cpp_project/`: The C++ implementation. Uses standard library `std::ifstream`, `std::getline`, and `std::stod`. Compiled with `g++ -O3`.
- `rust_project/`: The Rust implementation. Uses `std::fs::File`, `io::BufReader`, string slice iterators (`split(',')`), and `f64::parse()`. Compiled with `cargo build --release`.
- `benchmark.sh`: A Bash script that builds both projects, generates the data if necessary, and uses the `time` command to measure and compare their execution times.

## How to Run the Benchmark

1. Make sure you have `g++`, `cargo`, and `python3` installed on your system.
2. Run the benchmark script:

```bash
./benchmark.sh
```

The script will handle data generation, compilation, and execution automatically.

## Expected Results
You will notice that the execution times may vary depending on your system, but generally, the Rust implementation runs significantly faster in this specific scenario. 

**Why?**
The C++ implementation relies on `std::stringstream` and `std::stod` for string splitting and conversion. These standard library features are notoriously slow due to heavy memory allocation per line and locale-aware checks during string-to-float conversions. 
Conversely, the Rust implementation uses extremely lightweight string slice iterators (`str::split`) that do not allocate memory, combined with a highly optimized float parsing routine, leading to far superior CPU utilization and speed.
