#include <iostream>
#include <future>
#include <chrono>
#include <thread>

int heavyComputation() {
    std::this_thread::sleep_for(std::chrono::seconds(2));
    return 42;
}

int main() {
    std::future<int> result = std::async(std::launch::async, heavyComputation);
    std::cout << "Doing other work...\n";
    std::cout << "The answer is: " << result.get() << "\n"; // Blocks until ready
}
