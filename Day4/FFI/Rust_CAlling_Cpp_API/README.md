# Rust Calling C++ API

This scenario demonstrates how a Rust program can call functions defined in a C++ library.

## How it works

1. **`build.rs` and the `cc` crate**: We use the `cc` crate in Rust as a build dependency. `build.rs` is executed before the main Rust compilation. It uses `cc` to compile our C++ code (`math_lib.cpp`) into a static library and automatically links it to our Rust executable.
2. **`extern "C"` in C++**: We wrap our C++ functions inside an `extern "C"` block. This ensures that the C++ compiler uses the standard C ABI (Application Binary Interface) for these functions, avoiding C++ name mangling, so Rust can link to them.
3. **`extern "C"` in Rust**: In `main.rs`, we declare the C++ functions within an `extern "C"` block.
4. **`unsafe` block**: Whenever Rust calls a foreign function, it must be done inside an `unsafe` block. This is because the Rust compiler cannot guarantee that the C++ code follows Rust's memory safety rules.

## Building and Running

You can build and run this scenario using cargo:

```bash
cargo run
```

Cargo will first execute `build.rs` to compile `math_lib.cpp`, and then compile and link `main.rs`.
