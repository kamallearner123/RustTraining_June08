# Rust Parsing C++ Message

This scenario demonstrates how C++ can construct a structured message (a `struct`) and pass it to Rust to be parsed.

## How it works

1. **Memory Layout (`#[repr(C)]`)**: Rust does not guarantee a specific layout for its structs by default. To make a Rust struct compatible with C/C++, we must add the `#[repr(C)]` attribute. This forces the Rust compiler to use the same memory layout rules (alignment, padding, etc.) as a C compiler.
2. **C++ Struct**: In the C++ code, we define a struct with exactly the same fields and types as the Rust struct. `int` in C++ corresponds to `i32` in Rust, and `double` corresponds to `f64`.
3. **Passing Pointers**: C++ creates the struct and passes a pointer to it to the Rust function. The Rust function signature accepts a `*const CppMessage` (a raw, immutable pointer).
4. **Safety in Rust**: Rust receives a raw pointer from C++. Rust cannot guarantee that this pointer is valid (it could be null or point to garbage). Therefore, dereferencing this pointer in Rust (`&*msg_ptr`) requires an `unsafe` block. Inside the block, Rust safely reads the fields.

## Building and Running

Simply run `make` in this directory.

```bash
make
./main
```

The Makefile will compile the Rust library and the C++ executable, linking them together.
