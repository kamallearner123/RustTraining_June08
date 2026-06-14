#include <iostream>
#include <vector>
#include <chrono>

int main() {
    const int LIMIT = 100'000'000;
    std::cout << "Calculating primes up to " << LIMIT << "..." << std::endl;

    auto start_time = std::chrono::high_resolution_clock::now();

    // std::vector<bool> is space-optimized (1 bit per element) in C++
    std::vector<bool> is_prime(LIMIT + 1, true);
    is_prime[0] = false;
    is_prime[1] = false;

    for (int p = 2; p * p <= LIMIT; p++) {
        if (is_prime[p]) {
            for (int i = p * p; i <= LIMIT; i += p) {
                is_prime[i] = false;
            }
        }
    }

    int prime_count = 0;
    for (int p = 2; p <= LIMIT; p++) {
        if (is_prime[p]) {
            prime_count++;
        }
    }

    auto end_time = std::chrono::high_resolution_clock::now();
    std::chrono::duration<double> diff = end_time - start_time;

    std::cout << "Sieve complete!" << std::endl;
    std::cout << "Primes found: " << prime_count << std::endl;
    std::cout << "C++ Prime Sieve Time: " << diff.count() << " seconds" << std::endl;

    return 0;
}
