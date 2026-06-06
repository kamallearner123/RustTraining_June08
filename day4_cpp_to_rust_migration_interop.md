# Day 4: C++ to Rust Migration + Interoperability

## Overview
You don't have to rewrite everything at once. Day 4 is focused on practical migration strategies, calling C++ from Rust (and vice versa) using FFI, understanding when to use `unsafe` Rust, and evaluating performance differences.

---

## Session 1: Migration Strategy
### Introduction
A complete rewrite is often too risky. We discuss incremental migration, deciding what *not* to migrate, and using modern C++23 features to bridge the gap before porting.

### Important Points
- **When NOT to migrate:** If a C++ module is stable, well-tested, isolated, and rarely updated, leave it as C++. Focus migration on high-churn, security-critical, or multi-threaded code.
- **Incremental Migration:** Build Rust as a static library (`.a`) or shared object (`.so`) and link it into the C++ build system (CMake).
- **Strangler Fig Pattern:** Gradually replace specific C++ modules with Rust modules, keeping the external interfaces intact.

---

## Session 2: C++ ↔ Rust Interop (FFI)
### Introduction
Foreign Function Interface (FFI) allows different languages to communicate. We will use the C ABI as the lingua franca between C++ and Rust, and automate binding generation.

### Important Points
- **Extern "C":** Both C++ and Rust need to disable name mangling and use the standard C ABI to talk to each other.
- **Bindgen:** A Rust tool that parses C/C++ headers and automatically generates Rust FFI declarations.
- **Cbindgen:** A tool that parses Rust code and generates C/C++ headers for Rust functions exported with `#[no_mangle] extern "C"`.

### Code Example
**Rust exporting a function:**
```rust
// lib.rs
#[no_mangle]
pub extern "C" fn process_data(data: *const i32, length: usize) -> i32 {
    let slice = unsafe { std::slice::from_raw_parts(data, length) };
    slice.iter().sum()
}
```
**C++ calling the Rust function:**
```cpp
// main.cpp
extern "C" int process_data(const int* data, size_t length);

int main() {
    int arr[] = {1, 2, 3, 4};
    int sum = process_data(arr, 4);
    return 0;
}
```

---

## Session 3: Unsafe Rust
### Introduction
Rust's safety checks are sometimes too restrictive for low-level systems programming or FFI. `unsafe` is an "escape hatch" where the programmer takes responsibility for memory safety.

### Important Points
- **What Unsafe Does:** It allows dereferencing raw pointers, calling unsafe functions (like FFI), modifying static mutable variables, and implementing unsafe traits.
- **What Unsafe Does NOT Do:** It does not turn off the borrow checker. It does not disable type checking.
- **Best Practice:** Keep `unsafe` blocks as small as possible and wrap them in safe, heavily tested abstractions.

---

## Session 4: Performance Comparison
### Introduction
We analyze benchmarks between C++ and Rust, focusing on memory footprint, execution speed, and the cost of FFI boundaries.

### Important Points
- **Execution Speed:** Generally on par. Both use LLVM as the backend optimizer.
- **Memory Footprint:** Rust binaries can be slightly larger due to static linking and monomorphization of generics, but runtime memory usage is identical or better (no hidden allocations).
- **FFI Cost:** Calling across the FFI boundary has almost zero overhead (it's just a function call), but converting complex data structures (like `std::string` to `String`) incurs allocation and copying costs.

---

## Session 5: Real Migration Case Study
### Introduction
We walk through a real-world scenario of migrating a legacy C++ network parsing module into a Rust microservice.

### Important Points
- **Error Reduction:** Highlighting how Rust's `match` statements eradicated unhandled edge cases in the network protocol.
- **CI/CD Integration:** How Cargo tests and CMake were unified in the build pipeline.

---

## Hands-on Lab
1. **Wrap a C++ Library:** Use `bindgen` to generate Rust bindings for a provided legacy C++ Math library (`lab4/legacy_math.hpp`). Call the C++ functions from a Rust binary.
2. **Hybrid System:** Write a Rust library that performs string manipulation. Expose it using `#[no_mangle] extern "C"` and use `cbindgen` to generate the header. Call this Rust library from a C++ main application.
3. **Encapsulate Unsafe Code:** You are given an unsafe Rust function that manipulates a raw pointer buffer. Wrap this unsafe code in a safe Rust struct that guarantees no out-of-bounds access.

---

## Multiple Choice Questions (Homework)

1. What is the recommended strategy for migrating a massive, active legacy C++ codebase to Rust?
<details><summary>View Answer</summary>
<b>Answer:</b> Incremental migration (Strangler Fig pattern), replacing high-risk or new modules first.
</details>

2. Which Application Binary Interface (ABI) is standardly used for communication between C++ and Rust?
<details><summary>View Answer</summary>
<b>Answer:</b> The C ABI.
</details>

3. In Rust, what attribute is required on a function to prevent the compiler from altering its name, allowing it to be called from C++?
<details><summary>View Answer</summary>
<b>Answer:</b> `#[no_mangle]`
</details>

4. Which tool is used to automatically generate Rust FFI declarations from C/C++ header files?
<details><summary>View Answer</summary>
<b>Answer:</b> `bindgen`
</details>

5. Which tool generates C/C++ header files from Rust source code?
<details><summary>View Answer</summary>
<b>Answer:</b> `cbindgen`
</details>

6. What is the primary purpose of the `unsafe` keyword in Rust?
<details><summary>View Answer</summary>
<b>Answer:</b> To opt out of specific compiler safety checks, such as dereferencing raw pointers, usually required for FFI or low-level systems programming.
</details>

7. Does the `unsafe` keyword turn off Rust's borrow checker?
<details><summary>View Answer</summary>
<b>Answer:</b> No, it only allows specific extra actions like dereferencing raw pointers; standard borrowing rules still apply to references.
</details>

8. What is the performance cost of a simple function call across the C++ / Rust FFI boundary (assuming primitive types)?
<details><summary>View Answer</summary>
<b>Answer:</b> Practically zero overhead; it is equivalent to a standard C function call.
</details>

9. When migrating, what is a major performance trap to watch out for at the FFI boundary?
<details><summary>View Answer</summary>
<b>Answer:</b> The cost of converting or copying complex data structures (like standard library strings or vectors) between the two languages.
</details>

10. What is a key benefit of wrapping an `unsafe` FFI call inside a safe Rust function?
<details><summary>View Answer</summary>
<b>Answer:</b> It contains the risk and provides a safe, idiomatic API to the rest of the Rust codebase, relying on the compiler for callers.
</details>
