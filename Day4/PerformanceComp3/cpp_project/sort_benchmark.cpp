#include <iostream>
#include <vector>
#include <algorithm>
#include <chrono>
#include <random>

int main() {
    const int NUM_ELEMENTS = 50'000'000;
    std::cout << "Generating " << NUM_ELEMENTS << " random integers..." << std::endl;

    std::vector<int> data(NUM_ELEMENTS);
    
    // Fast PRNG for generating data
    std::mt19937 rng(42); // Fixed seed for reproducible data
    std::uniform_int_distribution<int> dist;
    
    for (int i = 0; i < NUM_ELEMENTS; ++i) {
        data[i] = dist(rng);
    }

    std::cout << "Starting C++ std::sort..." << std::endl;

    auto start_time = std::chrono::high_resolution_clock::now();
    
    std::sort(data.begin(), data.end());
    
    auto end_time = std::chrono::high_resolution_clock::now();
    std::chrono::duration<double> diff = end_time - start_time;

    std::cout << "Sorting complete!" << std::endl;
    std::cout << "C++ Sorting Time: " << diff.count() << " seconds" << std::endl;
    
    // Just print something to prevent over-optimization
    std::cout << "First 5 elements: ";
    for(int i=0; i<5; ++i) std::cout << data[i] << " ";
    std::cout << "\nLast 5 elements: ";
    for(int i=NUM_ELEMENTS-5; i<NUM_ELEMENTS; ++i) std::cout << data[i] << " ";
    std::cout << std::endl;

    return 0;
}
