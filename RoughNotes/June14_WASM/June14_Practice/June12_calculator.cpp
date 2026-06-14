#include <iostream>
#include <string>

int add(int a, int b) { return a + b; }
int subtract(int a, int b) { return a - b; }
long long factorial(int n) {
    if (n <= 1) return 1;
    return n * factorial(n - 1);
}

int main() {
    std::cout << "=== Native Calculator ===" << std::endl;
    std::cout << "add(10, 5)       = " << add(10, 5) << std::endl;
    std::cout << "subtract(10, 5)  = " << subtract(10, 5) << std::endl;
    std::cout << "factorial(10)    = " << factorial(10) << std::endl;
    return 0;
}
