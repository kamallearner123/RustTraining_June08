#!/bin/bash

# Exit on error
set -e

# Navigate to the script's directory
cd "$(dirname "$0")"

echo "================================================="
echo "  Performance Comparison 2: Hash Map Operations  "
echo "================================================="

# 1. Generate Data
if [ ! -f "words.txt" ]; then
    echo "Generating word data..."
    python3 generate_data.py
else
    echo "Data file words.txt already exists. Skipping generation."
fi

# 2. Compile C++
echo ""
echo "Compiling C++ Project..."
cd cpp_project
g++ -O3 main.cpp -o main
cd ..

# 3. Compile Rust
echo ""
echo "Compiling Rust Project..."
cd rust_project
cargo build --release
cd ..

echo ""
echo "================================================="
echo "  Executing Benchmarks                           "
echo "================================================="

# 4. Run C++ Benchmark
echo ""
echo "[ C++ Execution Time ]"
time ./cpp_project/main

# 5. Run Rust Benchmark
echo ""
echo "[ Rust Execution Time ]"
time ./rust_project/target/release/rust_project

echo ""
echo "Benchmark completed."
