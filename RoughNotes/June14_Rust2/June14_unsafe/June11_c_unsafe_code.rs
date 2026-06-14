// Declare the external functions from the C++ library
unsafe extern "C" {
    fn cpp_hello();
    fn cpp_multiply(a: i32, b: i32) -> i32;
}

fn main() {
    println!("Rust: Calling cpp_hello()...");
    // Any call to an external function requires an unsafe block
    // because the Rust compiler cannot guarantee the memory safety of external code.
    unsafe {
        cpp_hello();
    }

    println!("Rust: Calling cpp_multiply()...");
    unsafe {
        let result = cpp_multiply(10, 20);
        println!("Rust: Result from C++: {}", result);
    }
}

