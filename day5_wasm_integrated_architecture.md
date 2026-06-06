# Day 5: Web Assembly (WASM) + Integrated Architecture

## Overview
The final day bridges backend systems to portable execution environments. We explore WebAssembly (WASM), how to compile C++ and Rust to WASM, running WASM outside the browser via WASI, and architecting an end-to-end system combining all three technologies.

---

## Session 1: WASM Fundamentals
### Introduction
WebAssembly is a binary instruction format for a stack-based virtual machine. It was designed as a portable compilation target for programming languages, enabling deployment on the web and edge devices.

### Important Points
- **Architecture:** WASM is not assembly language for a specific physical machine; it's a virtual instruction set architecture (ISA).
- **Execution Model:** It runs in a memory-safe, sandboxed execution environment.
- **Why WASM for Systems?** It offers near-native performance, tiny binary sizes, and extreme portability. Write once, run securely anywhere (Edge, Cloud, Browser).

---

## Session 2: Toolchain
### Introduction
We look at how LLVM makes it possible to target WASM from multiple languages. We will explore Emscripten for C++ and `wasm-pack` for Rust.

### Important Points
- **Emscripten:** A complete compiler toolchain to WebAssembly for C/C++. It emulates POSIX APIs, making it easier to port legacy C++ apps to the web.
- **Rust to WASM:** Rust has first-class support for WASM via the `wasm32-unknown-unknown` target. `wasm-pack` is the standard tool to build and package Rust-generated WebAssembly.
- **wasm-bindgen:** A tool that facilitates high-level interactions between WASM modules and JavaScript.

### Code Example
**Rust (compiled via wasm-pack):**
```rust
use wasm_bindgen::prelude::*;

// This exposes the function to the JS/WASM host environment
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {} from WebAssembly!", name)
}
```

---

## Session 3: WASI & Runtime
### Introduction
WASM isn't just for browsers anymore. The WebAssembly System Interface (WASI) standardizes how WASM modules interact with the operating system (files, network, etc.).

### Important Points
- **WASI (WebAssembly System Interface):** A modular system interface for WebAssembly. It provides capabilities like file I/O and clock access in a secure, sandboxed way.
- **Runtimes:** `Wasmtime` (developed by Bytecode Alliance) and `WasmEdge` are lightweight, standalone WASM runtimes that execute WASI modules outside the browser.
- **Edge Computing:** WASM is becoming the standard for serverless functions at the edge due to its instant startup times (microseconds compared to Docker's milliseconds/seconds).

---

## Session 4: Performance & Security
### Introduction
We evaluate why platforms are shifting towards WASM for plugin architectures and microservices.

### Important Points
- **Startup Latency:** WASM modules start almost instantly because there is no OS boot overhead (unlike VMs or heavy containers).
- **Sandboxing Benefits:** The host system explicitly grants capabilities (like reading a specific folder). A compromised WASM module cannot break out and access the host OS.
- **Production Use Cases:** Proxy filters (Envoy), database user-defined functions (UDFs), and edge serverless.

---

## Session 5: End-to-End System Design
### Introduction
Putting it all together. How do we design a modern architecture that leverages legacy C++, safe Rust, and portable WASM?

### Important Points
- **The Pipeline:** C++ (Legacy heavy lifting) → Rust (Safe orchestration and new logic via FFI) → WASM (Portable, sandboxed execution at the edge).
- **Microservices:** Packaging Rust/WASM binaries into distroless containers for Kubernetes deployments.

---

## Final Hands-on Project
**Build an Integrated System:**
You are tasked with building a processing pipeline that incorporates all technologies learned this week.
1. **C++ Module:** Take a provided legacy C++ image processing algorithm (`final/image_filter.cpp`).
2. **Rust Module:** Write a Rust core application that uses FFI to load the C++ algorithm safely. The Rust app should handle concurrency and safe memory management.
3. **WASM Module:** Compile a separate Rust module to WASM targeting WASI. This module will act as an edge-compute plugin that applies a lightweight transformation to the data before it reaches the Rust core.
4. **Integration:** Run the WASM plugin using `Wasmtime` from within the Rust core, passing the resulting data to the C++ FFI backend.

---

## Multiple Choice Questions (Homework)

1. What is WebAssembly (WASM)?
<details><summary>View Answer</summary>
<b>Answer:</b> A portable, binary instruction format for a stack-based virtual machine, designed as a compilation target for high-level languages.
</details>

2. Which underlying compiler infrastructure enables both C++ and Rust to compile to WebAssembly?
<details><summary>View Answer</summary>
<b>Answer:</b> LLVM
</details>

3. What is Emscripten?
<details><summary>View Answer</summary>
<b>Answer:</b> A compiler toolchain specifically designed to compile C and C++ code to WebAssembly, providing POSIX emulation.
</details>

4. In the Rust ecosystem, which tool is commonly used to build, package, and publish WebAssembly modules?
<details><summary>View Answer</summary>
<b>Answer:</b> `wasm-pack`
</details>

5. What does WASI stand for?
<details><summary>View Answer</summary>
<b>Answer:</b> WebAssembly System Interface
</details>

6. What problem does WASI solve for WebAssembly?
<details><summary>View Answer</summary>
<b>Answer:</b> It provides a standard, secure way for WASM modules to access operating system features (like files and networking) outside of a web browser.
</details>

7. Name a popular standalone WebAssembly runtime used for executing WASI modules.
<details><summary>View Answer</summary>
<b>Answer:</b> `Wasmtime` (or `WasmEdge`, `Wasmer`)
</details>

8. Why is WebAssembly highly attractive for Edge Computing / Serverless architectures compared to Docker containers?
<details><summary>View Answer</summary>
<b>Answer:</b> Extreme portability and near-instant startup latency (microseconds vs. milliseconds/seconds).
</details>

9. How does WebAssembly handle security and access to the host file system?
<details><summary>View Answer</summary>
<b>Answer:</b> It runs in a strict sandbox; access to the file system must be explicitly granted by the host environment via capabilities.
</details>

10. In the proposed End-to-End architecture (C++ → Rust → WASM), what role does Rust play between the other two technologies?
<details><summary>View Answer</summary>
<b>Answer:</b> It acts as the safe orchestrator, using FFI to interface with legacy C++ and executing WASM modules securely within its runtime environment.
</details>
