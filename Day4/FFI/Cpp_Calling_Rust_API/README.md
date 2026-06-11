# C++ Calling Rust API

This scenario demonstrates how a C++ program can call functions defined in a Rust library.

## How it works

1. **Rust staticlib**: We compile the Rust code as a `staticlib` by setting `crate-type = ["staticlib"]` in `rust_lib/Cargo.toml`. This produces an archive file (e.g., `librust_lib.a`) that contains all the compiled Rust code and its dependencies.
2. **`#[no_mangle]`**: In Rust, we use the `#[no_mangle]` attribute to prevent the Rust compiler from altering the name of the function. This ensures the function name remains the same in the compiled library, so C++ can find it.
3. **`extern "C"`**: We mark the Rust functions with `extern "C"` to tell the Rust compiler to use the C Application Binary Interface (ABI). This guarantees that the function's arguments and return values are passed in a way that C/C++ expects.
4. **C++ declarations**: In the C++ code (`main.cpp`), we declare the Rust functions inside an `extern "C"` block. This tells the C++ compiler to look for these functions using the C ABI, avoiding C++ name mangling.

## Building and Running

Simply run `make` in this directory.

```bash
make
./main
```

The Makefile will:
1. Run `cargo build` in the `rust_lib` directory.
2. Compile `main.cpp` and link it against `rust_lib/target/debug/librust_lib.a`.

*Note: Depending on your system and Rust standard library dependencies, you might need to link additional system libraries like `-lpthread`, `-ldl`, and `-lm` (which the Makefile already does).*
