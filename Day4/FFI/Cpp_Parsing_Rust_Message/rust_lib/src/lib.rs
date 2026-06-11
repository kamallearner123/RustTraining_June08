// Define the struct with a C-compatible memory layout
#[repr(C)]
pub struct RustMessage {
    pub id: i32,
    pub status: bool,
    pub value: f32,
}

// Function to construct the message in Rust and return it to C++
// We return the struct by value, which is safe because it's a simple,
// self-contained #[repr(C)] data structure without complex memory management.
#[no_mangle]
pub extern "C" fn get_rust_message() -> RustMessage {
    println!("Rust: Constructing message...");
    RustMessage {
        id: 100,
        status: true,
        value: 9.81,
    }
}
