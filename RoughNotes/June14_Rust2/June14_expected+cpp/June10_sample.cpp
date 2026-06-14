#include <iostream>
#include <expected>
#include <string>

std::expected<double, std::string> divide(double a, double b) {
    if (b == 0.0) return std::unexpected("Divide by zero!");
    return a / b;
}

int main() {
    auto result = divide(10.0, 0.0);
    
    // Explicit, safe error checking without try/catch blocks
    if (result) {
        std::cout << "Result: " << *result << "\n";
    } else {
        std::cout << "Failed: " << result.error() << "\n";
    }
}
