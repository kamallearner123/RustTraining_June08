#include <iostream>

// Declare the external functions from the Rust library
extern "C" {
    void rust_hello();
    int rust_add(int a, int b);
}

int main() {
    std::cout << "C++: Calling rust_hello()..." << std::endl;
    rust_hello();

    std::cout << "C++: Calling rust_add()..." << std::endl;
    int result = rust_add(10, 20);
    std::cout << "C++: Result from Rust: " << result << std::endl;

    return 0;
}
