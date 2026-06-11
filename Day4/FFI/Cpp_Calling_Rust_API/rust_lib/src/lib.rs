#[no_mangle]
pub extern "C" fn rust_hello() {
    println!("Hello from Rust!");
}

#[no_mangle]
pub extern "C" fn rust_add(a: i32, b: i32) -> i32 {
    println!("Rust is calculating {} + {}...", a, b);
    a + b
}
