// Define the struct with a C-compatible memory layout
#[repr(C)]
pub struct CppMessage {
    pub id: i32,
    pub payload: f64,
}

// Function to parse the message passed from C++
// It takes a raw pointer to the CppMessage struct
#[no_mangle]
pub extern "C" fn parse_cpp_message(msg_ptr: *const CppMessage) {
    if msg_ptr.is_null() {
        println!("Rust: Received a null pointer!");
        return;
    }

    // Safety: We must trust that C++ passed a valid pointer to a CppMessage
    unsafe {
        let msg = &*msg_ptr;
        println!("Rust: Parsed message -> ID: {}, Payload: {}", msg.id, msg.payload);
    }
}
