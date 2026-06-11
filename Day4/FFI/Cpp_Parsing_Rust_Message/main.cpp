#include <iostream>
#include <iomanip>

// Struct definition matching the Rust #[repr(C)] struct
// Note: 'bool' in C++ is generally 1 byte, which matches Rust's 'bool' in #[repr(C)].
struct RustMessage {
    int id;
    bool status;
    float value;
};

// Declare the external Rust function
extern "C" {
    RustMessage get_rust_message();
}

int main() {
    std::cout << "C++: Calling Rust to get message..." << std::endl;
    
    // Receive the message by value from Rust
    RustMessage msg = get_rust_message();

    std::cout << "C++: Received message from Rust:" << std::endl;
    std::cout << "  - ID: " << msg.id << std::endl;
    std::cout << "  - Status: " << std::boolalpha << msg.status << std::endl;
    std::cout << "  - Value: " << msg.value << std::endl;

    return 0;
}
