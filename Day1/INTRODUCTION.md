# Introduction: Systems Programming Fundamentals

Before diving into advanced debugging techniques, it is essential to establish a firm understanding of what happens when source code is transformed into a running application and how the operating system manages it. This document serves as a "warm-up" for the concepts explored in Day 1.

---

## 1. Program vs. Process

### Program
A **program** is a passive entity—it is a file stored on disk (e.g., an ELF binary in Linux) containing a list of machine instructions and static data. It does nothing until it is executed.

### Process
A **process** is an active entity—it is a program in execution. When an operating system launches a program, it allocates memory, assigns a unique Process ID (PID), and creates a virtual address space. A single program can be instantiated as multiple independent processes simultaneously.

---

## 2. The Compilation Process (C/C++)

The journey from human-readable source code (`.c` or `.cpp`) to an executable binary involves four distinct stages, often managed automatically by the compiler (`gcc` or `clang`).

1. **Preprocessing:** 
   - Handles all directives starting with `#` (like `#include` and `#define`).
   - Strips comments and expands macros.
   - Outputs a pure C/C++ translation unit (`.i` or `.ii`).
2. **Compilation:**
   - Translates the preprocessed source code into architecture-specific Assembly code (`.s`).
   - This is where syntax checking and most optimizations (like `-O2`) occur.
3. **Assembly:**
   - Converts the human-readable Assembly instructions into raw machine code (Object file: `.o`).
   - The code is now binary but not yet executable because it lacks exact memory addresses for external functions.
4. **Linking:**
   - The Linker (`ld`) takes one or more Object files and combines them into a single Executable and Linkable Format (ELF) binary.
   - It resolves symbols (e.g., matching a call to `printf` with the actual implementation in `libc.so`).

> **Debugging Context:** The `-g` flag tells the compiler to embed DWARF debugging information during the compilation stage, mapping the final binary instructions back to the original source code lines.

---

## 3. The Memory Layout of a Process

When a process is loaded into memory, the operating system gives it an isolated "Virtual Address Space." This space is divided into several logical segments:

```text
High Memory Addresses (e.g., 0xFFFFFFFF)

+-----------------------+
|        Stack          |  <-- Grows downwards
| (Local vars, frames)  |
+-----------------------+
|          |            |
|          v            |
|                       |
|          ^            |
|          |            |
+-----------------------+
|        Heap           |  <-- Grows upwards
|  (Dynamic memory)     |
+-----------------------+
|    BSS Segment        |  <-- Uninitialized globals
+-----------------------+
|    Data Segment       |  <-- Initialized globals
+-----------------------+
|    Text Segment       |  <-- Executable code / Read-Only
+-----------------------+

Low Memory Addresses (e.g., 0x00000000)
```

### Breakdown of Segments:
*   **Text Segment (Code):** Contains the compiled machine instructions. It is usually marked as Read-Only to prevent the process from accidentally modifying its own execution instructions.
*   **Data Segment:** Stores initialized global and static variables (e.g., `int count = 5;`).
*   **BSS Segment (Block Started by Symbol):** Stores uninitialized global and static variables. The OS efficiently initializes this entire block to zero at startup.
*   **Heap:** Used for dynamic memory allocation at runtime (via `malloc` in C or `new` in C++). It grows upwards. If a developer forgets to `free`/`delete` memory here, it causes a **Memory Leak**.
*   **Stack:** Used for function call frames, local variables, and control flow (return addresses). It operates on a Last-In-First-Out (LIFO) basis and grows downwards. Exhausting this space (e.g., via infinite recursion) causes a **Stack Overflow**.

---

## 4. Why Does This Matter for Debugging?

Understanding this structure is critical when applications crash:
*   **Segmentation Fault (SIGSEGV):** Occurs when a process tries to access a memory segment it isn't allowed to (like writing to the Read-Only Text Segment, or dereferencing a `NULL` pointer at address `0x0`).
*   **Data Races:** When multiple threads are spawned within a single process, they all share the *same* Heap, Data, and BSS segments, but each gets its own unique Stack. Shared state without synchronization leads to data races.
*   **Core Dumps:** A core dump is simply a snapshot of these memory segments (Stack, Heap, Data) saved to disk at the exact moment the process died. 

With this foundation, you are now ready to tackle the advanced debugging scenarios presented in **[README.md](README.md)** and **[README_C.md](README_C.md)**!
