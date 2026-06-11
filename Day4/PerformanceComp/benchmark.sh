#!/bin/bash

# Ensure data exists
if [ ! -f "data.csv" ]; then
    echo "Data file not found. Generating data..."
    python3 generate_data.py
fi

echo ""
echo "========================================"
echo "Compiling C++ Project (Release)..."
echo "========================================"
cd cpp_project
make clean
make
cd ..

echo ""
echo "========================================"
echo "Compiling Rust Project (Release)..."
echo "========================================"
cd rust_project
cargo build --release
cd ..

echo ""
echo "========================================"
echo "Running C++ Benchmark"
echo "========================================"
cd cpp_project
time ./process_data
cd ..

echo ""
echo "========================================"
echo "Running Rust Benchmark"
echo "========================================"
cd rust_project
time ./target/release/rust_project
cd ..

echo ""
echo "========================================"
echo "Benchmark Complete"
echo "========================================"
