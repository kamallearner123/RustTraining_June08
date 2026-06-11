#include <iostream>

// Struct definition matching the Rust #[repr(C)] struct
struct CppMessage {
    int id;
    double payload;
};

// Declare the external Rust function
extern "C" {
    void parse_cpp_message(const CppMessage* msg);
}

int main() {
    // Create a message in C++ memory
    CppMessage msg;
    msg.id = 42;
    msg.payload = 3.14159;

    std::cout << "C++: Sending message (ID: " << msg.id << ", Payload: " << msg.payload << ") to Rust..." << std::endl;

    // Pass the pointer to Rust
    parse_cpp_message(&msg);

    return 0;
}
