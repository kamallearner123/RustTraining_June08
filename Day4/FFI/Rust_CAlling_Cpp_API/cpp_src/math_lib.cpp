#include "math_lib.h"
#include <iostream>

extern "C" {
    void cpp_hello() {
        std::cout << "Hello from C++!" << std::endl;
    }

    int cpp_multiply(int a, int b) {
        std::cout << "C++ is calculating " << a << " * " << b << "..." << std::endl;
        return a * b;
    }
}
