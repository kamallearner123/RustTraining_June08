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

11. What is the Strangler Fig pattern in the context of system migration?
<details><summary>View Answer</summary>
<b>Answer:</b> Incrementally rewriting specific modules of a legacy system while keeping the rest intact, slowly "strangling" the old codebase until it's completely replaced.
</details>

12. Why is name mangling a problem when calling C++ functions from Rust?
<details><summary>View Answer</summary>
<b>Answer:</b> C++ changes function names to encode type information, making it impossible for Rust to link to the function by its original name unless `extern "C"` is used.
</details>

13. What is the type of a raw, immutable pointer in Rust?
<details><summary>View Answer</summary>
<b>Answer:</b> `*const T`
</details>

14. How do you convert a Rust `&str` into a C-compatible null-terminated string?
<details><summary>View Answer</summary>
<b>Answer:</b> Using the `std::ffi::CString` type.
</details>

15. What type represents a borrowed C-compatible string in Rust?
<details><summary>View Answer</summary>
<b>Answer:</b> `std::ffi::CStr`
</details>

16. What does `bindgen` need in order to generate Rust bindings from C++?
<details><summary>View Answer</summary>
<b>Answer:</b> The C/C++ header files and the clang compiler toolchain.
</details>

17. Can you safely pass a Rust `Vec<T>` directly across an FFI boundary to a C++ function?
<details><summary>View Answer</summary>
<b>Answer:</b> No, the memory layout of `Vec` is specific to Rust. You must pass a pointer to its buffer (`as_ptr()`) and its length.
</details>

18. What is the purpose of `std::mem::forget` in FFI?
<details><summary>View Answer</summary>
<b>Answer:</b> To prevent Rust from running the destructor on an object, often used when ownership of the memory is being transferred to C++.
</details>

19. How do you construct a safe Rust slice from a raw pointer and a length received from C++?
<details><summary>View Answer</summary>
<b>Answer:</b> Using `std::slice::from_raw_parts` inside an `unsafe` block.
</details>

20. What trait must a Rust struct derive to ensure its memory layout matches the C equivalent?
<details><summary>View Answer</summary>
<b>Answer:</b> `#[repr(C)]`
</details>

21. What happens if a Rust panic crosses an FFI boundary into C++ code?
<details><summary>View Answer</summary>
<b>Answer:</b> It causes Undefined Behavior and will likely crash the process. Panics must be caught using `catch_unwind` before crossing the boundary.
</details>

22. What does monomorphization mean in the context of Rust generics?
<details><summary>View Answer</summary>
<b>Answer:</b> The compiler generates a unique copy of the function/type for each specific type it is used with, which can increase binary size but maximizes execution speed.
</details>

23. When profiling a hybrid C++/Rust application, what tool can provide a unified flame graph spanning both languages?
<details><summary>View Answer</summary>
<b>Answer:</b> `perf`, since both use the system ABI and DWARF debug info.
</details>

24. What is the typical bottleneck in a heavily integrated hybrid C++/Rust application?
<details><summary>View Answer</summary>
<b>Answer:</b> The FFI boundary, specifically the overhead of translating and copying complex data types between the two memory domains.
</details>

25. Can `cbindgen` generate C++ classes with virtual methods from Rust structs?
<details><summary>View Answer</summary>
<b>Answer:</b> No, FFI relies on the C ABI, which only supports plain structs and global functions, not C++ objects or virtual tables.
</details>

26. What does `unsafe { ... }` block specifically allow you to do with an `extern` function?
<details><summary>View Answer</summary>
<b>Answer:</b> It allows you to call the function, as the Rust compiler cannot verify the memory safety of external code.
</details>

27. What is `std::os::raw::c_int` used for in Rust?
<details><summary>View Answer</summary>
<b>Answer:</b> It guarantees that the Rust integer type perfectly matches the size and alignment of a standard `int` in C for the target platform.
</details>

28. How do you allocate memory in Rust that needs to be manually freed by C++ code?
<details><summary>View Answer</summary>
<b>Answer:</b> Allocate it using Rust's global allocator (e.g., `Box::into_raw`), pass the pointer to C++, and provide an `extern "C"` function that C++ can call to free the pointer using `Box::from_raw`.
</details>

29. Why might a Rust binary be statically larger than an equivalent C++ binary?
<details><summary>View Answer</summary>
<b>Answer:</b> Rust statically links its standard library by default, whereas C++ usually dynamically links against `libstdc++`.
</details>

30. In migration, why is it recommended to rewrite leaf nodes of the dependency graph first?
<details><summary>View Answer</summary>
<b>Answer:</b> Leaf nodes (like utility libraries) don't depend on other C++ code, making them easiest to port to Rust and then expose back to the rest of the C++ codebase via FFI.
</details>
