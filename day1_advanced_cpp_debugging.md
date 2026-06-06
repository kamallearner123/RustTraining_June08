# Day 1: Advanced C++ Debugging on Linux (RHEL 9)

## Overview
Welcome to Day 1! Today's focus is on mastering debugging techniques in a Linux environment for C++ applications. We cover the transition from basic to advanced debugging, core dumps, memory diagnostics, and performance analysis. 

---

## Session 1: Debugging Fundamentals (Advanced View)
### Introduction
The purpose of this session is to distinguish between optimized (`-O2`, `-O3`) and debug (`-g`) builds, understand how symbol tables and DWARF information map binary code to source code, and identify common memory corruption patterns (stack vs. heap).

### Important Points
- **Optimized vs Debug:** Optimizations change execution order and inline functions, making line-by-line debugging difficult. Always use `-g -O0` for the best debugging experience, or `-g -Og` for a balance.
- **DWARF Info:** This is the debugging data format used by ELF. It maps instruction pointers back to source code lines.
- **Stack vs Heap Corruption:** Stack overflows usually cause immediate faults (e.g., recursive calls), while heap corruption might cause delayed faults somewhere else in the application.

### Code Example
```cpp
#include <iostream>
#include <cstring>

// Heap corruption example
void heapCorruption() {
    char* buffer = new char[10];
    // Writing out of bounds!
    std::strcpy(buffer, "This string is way too long for 10 bytes");
    std::cout << buffer << std::endl;
    delete[] buffer; // Crash usually happens here, not on the strcpy
}
```

---

## Session 2: GDB Mastery
### Introduction
GDB is the standard debugger on Linux. This session aims to turn basic users into power users by leveraging advanced features like conditional breakpoints, hardware watchpoints, reverse debugging, and handling multi-threaded code.

### Important Points
- **Conditional Breakpoints:** Use `break [location] if [condition]` to stop only when needed.
- **Watchpoints:** Hardware watchpoints (`watch [var]`) stop execution whenever the value of a variable changes in memory.
- **Thread Debugging:** `info threads` lists all threads; `thread apply all bt` shows backtraces for every thread.

### Code Example
```cpp
#include <thread>
#include <vector>

void worker(int id) {
    int counter = 0;
    while(true) {
        counter++; // In GDB: watch counter if id == 2
    }
}

int main() {
    std::vector<std::thread> threads;
    for (int i = 0; i < 5; ++i) {
        threads.push_back(std::thread(worker, i));
    }
    for (auto& t : threads) t.join();
}
```

---

## Session 3: Crash & Core Dump Analysis
### Introduction
When an application crashes in production, you can't attach a debugger. Core dumps capture the memory state at the time of the crash. This session teaches how to enable them and extract actionable backtraces.

### Important Points
- **Enabling Core Dumps:** Use `ulimit -c unlimited` in the terminal. In RHEL 9, core dumps are typically managed by `systemd-coredump`. Use `coredumpctl list` and `coredumpctl gdb` to access them.
- **Post-Mortem:** You need the exact binary and debug symbols that generated the core dump to analyze it accurately.

---

## Session 4: Memory Debugging
### Introduction
Memory bugs (leaks, use-after-free, uninitialized reads) are the hardest to track down. We will utilize Valgrind and compiler-based Sanitizers (ASAN/UBSAN) to find these automatically.

### Important Points
- **Valgrind (memcheck):** An instrumentation framework. Very accurate but slows down execution by 10x-50x.
- **ASAN (AddressSanitizer):** Compiler instrumentation (`-fsanitize=address`). Fast enough for some test environments (2x slowdown). Finds bounds violations and use-after-free.
- **UBSAN (UndefinedBehaviorSanitizer):** Catches integer overflows, null pointer dereferences, etc. (`-fsanitize=undefined`).

### Code Example
```cpp
// Compile with: g++ -g -fsanitize=address mem_bug.cpp
int main() {
    int* array = new int[100];
    delete[] array;
    return array[1]; // ASAN Error: heap-use-after-free
}
```

---

## Session 5: Performance Debugging
### Introduction
Finding out why CPU usage is at 100% requires profiling. We will use Linux `perf` and Flame Graphs to visualize CPU cycles and pinpoint hot paths.

### Important Points
- **Perf:** Run `perf record -g ./app` to record call graphs, and `perf report` to view them.
- **Flame Graphs:** Visual representation of `perf` output. The x-axis shows the population (percentage of samples), and the y-axis shows stack depth.
- **Cache Misses:** Use `perf stat -e cache-misses ./app` to see if your data layout is thrashing the CPU cache.

---

## Hands-on Lab
1. **Multithreaded Crash:** You are provided with a crashing multithreaded service (`lab1/crash_service.cpp`). Use GDB `thread apply all bt` to find the exact thread and line causing a segmentation fault.
2. **Memory Leak Hunt:** Compile `lab1/leak_app.cpp` using `-fsanitize=address`. Run the application and capture the ASAN report. Identify the exact function leaking memory and fix the code.
3. **Core Dump Analysis:** Force a crash in a sample application. Use `coredumpctl` to locate the dump, load it in GDB, and print the local variables at the time of the crash.

---

## Multiple Choice Questions (Homework)

1. What compiler flag is strictly required to include DWARF debugging symbols in the output binary?
<details><summary>View Answer</summary>
<b>Answer:</b> `-g`
</details>

2. When using GDB, what command sets a breakpoint at line 42 only if variable `x` is greater than 10?
<details><summary>View Answer</summary>
<b>Answer:</b> `break 42 if x > 10`
</details>

3. Which type of memory corruption typically results in an immediate crash or segmentation fault?
<details><summary>View Answer</summary>
<b>Answer:</b> Stack Overflow / Stack Corruption
</details>

4. In GDB, what is the command to view the backtrace of all currently running threads?
<details><summary>View Answer</summary>
<b>Answer:</b> `thread apply all bt`
</details>

5. How do you temporarily allow unlimited core dump sizes for the current shell session in Linux?
<details><summary>View Answer</summary>
<b>Answer:</b> `ulimit -c unlimited`
</details>

6. In modern RHEL 9 systems, which tool is typically used to list and manage captured core dumps?
<details><summary>View Answer</summary>
<b>Answer:</b> `coredumpctl`
</details>

7. Which compiler flag enables AddressSanitizer (ASAN)?
<details><summary>View Answer</summary>
<b>Answer:</b> `-fsanitize=address`
</details>

8. Valgrind’s Memcheck tool is excellent for finding memory leaks, but what is its primary drawback compared to ASAN?
<details><summary>View Answer</summary>
<b>Answer:</b> It introduces massive performance overhead (often 10x-50x slowdown).
</details>

9. Which Linux tool is used to record CPU profiling data that can later be converted into a Flame Graph?
<details><summary>View Answer</summary>
<b>Answer:</b> `perf` (specifically `perf record`)
</details>

10. What does the y-axis (vertical height) represent in a CPU Flame Graph?
<details><summary>View Answer</summary>
<b>Answer:</b> Stack depth (call stack frames)
</details>
