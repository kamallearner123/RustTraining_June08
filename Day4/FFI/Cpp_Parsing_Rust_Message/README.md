# C++ Parsing Rust Message

This scenario demonstrates how Rust can construct a structured message (a `struct`) and pass it to C++ to be parsed. This is the inverse of the previous scenario.

## How it works

1. **Memory Layout (`#[repr(C)]`)**: Just like when passing data from C++ to Rust, passing data from Rust to C++ requires the `#[repr(C)]` attribute on the Rust struct to ensure a stable, C-compatible memory layout.
2. **Returning by Value**: In this simple example, we are returning the struct by value from the Rust function. This works safely because the struct only contains plain data types (no pointers or dynamically allocated memory). C/C++ ABIs dictate how small structs are returned (often in registers or on the stack).
3. **C++ Struct**: The C++ code defines a matching struct. It calls the Rust function, receives a copy of the struct, and safely reads the data.
4. **Boolean types**: Rust's `bool` type in a `#[repr(C)]` struct is generally equivalent to an 8-bit integer, which maps directly to the C++ `bool` type on most platforms.

## Building and Running

Simply run `make` in this directory.

```bash
make
./main
```

The Makefile will compile the Rust library and the C++ executable, linking them together.
