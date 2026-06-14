#!/bin/bash

# Exit on error
set -e

# Navigate to the script's directory
cd "$(dirname "$0")"

echo "================================================="
echo "  Performance Comparison 3: CPU Bound Tasks      "
echo "================================================="

# 1. Compile C++
echo ""
echo "Compiling C++ Projects (-O3)..."
mkdir -p cpp_project/bin
g++ -O3 cpp_project/sort_benchmark.cpp -o cpp_project/bin/sort_benchmark
g++ -O3 cpp_project/prime_benchmark.cpp -o cpp_project/bin/prime_benchmark

# 2. Compile Rust
echo ""
echo "Compiling Rust Projects (--release)..."
cd rust_project
cargo build --release
cd ..

echo ""
echo "================================================="
echo "  Executing Benchmark 1: Large Array Sorting     "
echo "================================================="

echo ""
echo "[ C++ std::sort Execution ]"
./cpp_project/bin/sort_benchmark

echo ""
echo "[ Rust slice::sort_unstable Execution ]"
./rust_project/target/release/sort_benchmark

echo ""
echo "================================================="
echo "  Executing Benchmark 2: Prime Number Sieve      "
echo "================================================="

echo ""
echo "[ C++ std::vector<bool> Execution ]"
./cpp_project/bin/prime_benchmark

echo ""
echo "[ Rust Vec<bool> Execution ]"
./rust_project/target/release/prime_benchmark

echo ""
echo "Benchmarks completed."
