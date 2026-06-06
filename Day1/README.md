# Day 1: Advanced C++ Debugging on Linux (RHEL 9)

## Overview
Welcome to Day 1! Today's focus is on mastering debugging techniques in a Linux environment for C++ applications. We cover the transition from basic to advanced debugging, core dumps, memory diagnostics, and performance analysis. This guide is heavily expanded with 15 practical examples to demonstrate exact failure modes and how to diagnose them.

---

## Session 1: Debugging Fundamentals (Advanced View)
### Introduction
The purpose of this session is to distinguish between optimized (`-O2`, `-O3`) and debug (`-g`) builds, understand how symbol tables and DWARF information map binary code to source code, and identify common memory corruption patterns (stack vs. heap). 

### Important Points
- **Optimized vs Debug:** Optimizations change execution order and inline functions, making line-by-line debugging difficult. Always use `-g -O0` for the best debugging experience, or `-g -Og` for a balance.
- **DWARF Info:** This is the debugging data format used by ELF binaries. It maps instruction pointers back to source code lines. You can inspect it using `readelf -w binary`.
- **Stack vs Heap Corruption:** Stack overflows usually cause immediate faults (e.g., recursive calls blowing the stack limit), while heap corruption might cause delayed faults somewhere else in the application, often far away from the actual bug.

### 1. Code Example: Stack Overflow
A classic stack overflow caused by infinite recursion. This will immediately cause a Segmentation Fault (SIGSEGV).
```cpp
#include <iostream>

void recursiveCall(int count) {
    // Missing base case
    int largeArray[1024]; // Consuming 4KB of stack per frame
    largeArray[0] = count;
    recursiveCall(count + 1);
}

int main() {
    std::cout << "Starting recursion..." << std::endl;
    recursiveCall(1); // Will crash with SIGSEGV (Segmentation fault)
    return 0;
}
```

### 2. Code Example: Heap Corruption (Out of Bounds)
Writing past the allocated boundary on the heap. The crash often doesn't happen during the write, but later during `delete[]` when the heap manager realizes its metadata is corrupted.
```cpp
#include <iostream>
#include <cstring>

void heapCorruption() {
    char* buffer = new char[10];
    // Writing 40 bytes into a 10-byte buffer!
    std::strcpy(buffer, "This string is way too long for 10 bytes");
    std::cout << buffer << std::endl;
    
    // Crash usually happens right here, as the memory allocator's 
    // internal bookkeeping structures were overwritten by strcpy.
    delete[] buffer; 
}
```

### 3. Code Example: Uninitialized Variable Read
Reading an uninitialized local variable means reading whatever garbage value was left on the stack. In debug builds, this might be zero. In release builds, it causes erratic behavior.
```cpp
#include <iostream>

int calculateResult(bool flag) {
    int result; // Uninitialized
    if (flag) {
        result = 42;
    }
    // If flag is false, we read garbage data
    return result; 
}

int main() {
    std::cout << "Result: " << calculateResult(false) << std::endl;
    return 0;
}
```

---

## Session 2: GDB Mastery
### Introduction
GDB is the standard debugger on Linux. This session aims to turn basic users into power users by leveraging advanced features like conditional breakpoints, hardware watchpoints, reverse debugging, and handling multi-threaded code.

### Important Points
- **Conditional Breakpoints:** Use `break [location] if [condition]` to stop only when needed. Saves hours in tight loops.
- **Watchpoints:** Hardware watchpoints (`watch [var]`) stop execution whenever the value of a variable changes in memory, without slowing down the application.
- **Reverse Debugging:** Using `record`, you can step backwards (`reverse-step`, `reverse-continue`) to find the exact moment state mutated.

### 4. Code Example: Conditional Breakpoints and Watchpoints
Imagine a loop running 10,000 times, but it only fails on the 9,999th iteration.
```cpp
#include <iostream>

int main() {
    int critical_value = 0;
    for (int i = 0; i < 10000; ++i) {
        if (i == 9999) {
            critical_value = 1; // BUG triggers here
        }
    }
    std::cout << critical_value << std::endl;
    return 0;
}
// In GDB:
// (gdb) break 6 if i == 9999
// OR
// (gdb) watch critical_value
```

### 5. Code Example: Data Race (Multi-threaded)
When two threads access shared memory without synchronization.
```cpp
#include <thread>
#include <vector>
#include <iostream>

int shared_counter = 0;

void increment() {
    for (int i = 0; i < 100000; ++i) {
        shared_counter++; // Not atomic! Data race!
    }
}

int main() {
    std::thread t1(increment);
    std::thread t2(increment);
    t1.join(); t2.join();
    // Result will be random, less than 200000
    std::cout << "Counter: " << shared_counter << std::endl;
}
// In GDB:
// (gdb) info threads
// (gdb) thread apply all bt
```

### 6. Code Example: Segfault in a Loop (Reverse Debugging)
A bug that corrupts a pointer midway through a process.
```cpp
#include <iostream>

int main() {
    int* ptr = new int(10);
    for (int i = 0; i < 100; ++i) {
        if (i == 50) {
            ptr = nullptr; // Pointer suddenly becomes null
        }
        if (i == 75) {
            *ptr = 20; // Segfault here!
        }
    }
}
// In GDB:
// (gdb) run (crashes at line 10)
// (gdb) record
// (gdb) reverse-continue (finds exactly when ptr became null)
```

---

## Session 3: Crash & Core Dump Analysis
### Introduction
When an application crashes in production, you can't attach a debugger. Core dumps capture the exact memory state at the time of the crash. This session teaches how to enable them and extract actionable backtraces.

### Important Points
- **Enabling Core Dumps:** Use `ulimit -c unlimited`. In RHEL 9, core dumps are managed by `systemd-coredump`. Use `coredumpctl list` to view them.
- **Loading Dumps:** `coredumpctl gdb <PID>` or `gdb ./binary ./core`.
- **Post-Mortem Requirements:** You MUST have the exact binary and debug symbols that generated the core dump to analyze it accurately.

### 7. Code Example: Null Pointer Dereference
A direct attempt to read or write to address `0x0`.
```cpp
struct Config {
    int version;
};

void printConfig(Config* cfg) {
    // If cfg is null, reading cfg->version causes a SIGSEGV at address 0x0.
    std::cout << "Version: " << cfg->version << std::endl; 
}

int main() {
    Config* myConfig = nullptr;
    printConfig(myConfig); // Segfault
    return 0;
}
```

### 8. Code Example: Double Free Error
Freeing the same block of memory twice corrupts the allocator's free list, causing `glibc` to abort the program (SIGABRT).
```cpp
#include <iostream>

int main() {
    int* data = new int[50];
    
    // ... complex logic ...
    delete[] data;
    
    // ... later in the code ...
    delete[] data; // Double free! Will generate a core dump via SIGABRT.
    
    return 0;
}
```

### 9. Code Example: Deadlock (Hang Dump Analysis)
The application doesn't crash, it just freezes. You can send `SIGABRT` to a hung process to force a core dump and analyze the thread states.
```cpp
#include <thread>
#include <mutex>

std::mutex mtx1;
std::mutex mtx2;

void threadA() {
    std::lock_guard<std::mutex> lock1(mtx1);
    std::this_thread::sleep_for(std::chrono::milliseconds(10));
    std::lock_guard<std::mutex> lock2(mtx2); // Waiting for mtx2
}

void threadB() {
    std::lock_guard<std::mutex> lock2(mtx2);
    std::this_thread::sleep_for(std::chrono::milliseconds(10));
    std::lock_guard<std::mutex> lock1(mtx1); // Waiting for mtx1 -> DEADLOCK
}

int main() {
    std::thread t1(threadA);
    std::thread t2(threadB);
    t1.join(); t2.join();
}
// To analyze: `kill -ABRT <pid>`, then `gdb ./binary ./core`, then `thread apply all bt`
```

---

## Session 4: Memory Debugging
### Introduction
Memory bugs (leaks, use-after-free, uninitialized reads) are the hardest to track down manually. We will utilize Valgrind and compiler-based Sanitizers (ASAN/UBSAN) to find these automatically at runtime.

### Important Points
- **Valgrind (memcheck):** An instrumentation framework. Very accurate but slows down execution by 10x-50x. Run with `valgrind --leak-check=full ./app`.
- **ASAN (AddressSanitizer):** Compiler instrumentation (`-fsanitize=address`). Fast enough for testing (2x slowdown). Finds bounds violations and use-after-free.
- **UBSAN (UndefinedBehaviorSanitizer):** Catches integer overflows, shifting out of bounds, etc. (`-fsanitize=undefined`).

### 10. Code Example: Classic Memory Leak
Memory is allocated but never released. Long-running services will eventually run out of RAM (OOM killed).
```cpp
void handleRequest() {
    int* sessionData = new int[1024]; // 4KB allocated
    // Process request...
    
    // Forgot to delete[] sessionData;
}

int main() {
    while(true) handleRequest(); // Leak loop
}
// Compile with: g++ -g -fsanitize=address leak.cpp
// ASAN will report: "Direct leak of 4096 byte(s)"
```

### 11. Code Example: Use-After-Free
Accessing memory after it has been returned to the heap. Extremely dangerous security vulnerability.
```cpp
#include <iostream>

int main() {
    int* array = new int[100];
    array[0] = 42;
    
    delete[] array; // Memory freed
    
    // Accessing freed memory! 
    // Without ASAN, this might silently print garbage or crash later.
    // WITH ASAN, it instantly halts and prints a beautiful trace.
    std::cout << array[0] << std::endl; 
    
    return 0;
}
// Compile with: g++ -g -fsanitize=address uaf.cpp
// ASAN will report: "heap-use-after-free"
```

### 12. Code Example: Signed Integer Overflow (UBSAN)
In C++, signed integer overflow is Undefined Behavior. Compilers optimize assuming it never happens.
```cpp
#include <iostream>
#include <limits>

int main() {
    int max_int = std::numeric_limits<int>::max(); // 2147483647
    
    // This is Undefined Behavior!
    int overflowed = max_int + 1; 
    
    std::cout << overflowed << std::endl;
    return 0;
}
// Compile with: g++ -g -fsanitize=undefined overflow.cpp
// UBSAN will report: "signed integer overflow: 2147483647 + 1 cannot be represented in type 'int'"
```

---

## Session 5: Performance Debugging
### Introduction
When CPU usage is at 100% or your application feels sluggish, guessing won't help. We will use Linux `perf` and Flame Graphs to visualize CPU cycles and pinpoint hot paths.

### Important Points
- **Perf Record & Report:** Run `perf record -g ./app` to record call graphs, and `perf report` to view an interactive TUI of CPU hogs.
- **Flame Graphs:** Visual representation of `perf` output. The x-axis shows the population (percentage of samples), and the y-axis shows stack depth.
- **Hardware Counters:** Use `perf stat` to measure low-level CPU metrics like L1/L2 cache misses and branch mispredictions.

### 13. Code Example: The 100% CPU Busy Loop
A thread stuck in a loop without sleeping, consuming an entire CPU core.
```cpp
#include <vector>
#include <numeric>

void heavyComputation() {
    volatile long sum = 0; // Volatile prevents compiler from optimizing loop away
    for(long i = 0; i < 10000000000; ++i) {
        sum += i;
    }
}

int main() {
    heavyComputation();
    return 0;
}
// Run: perf record -g ./app
// Perf will show exactly that heavyComputation() takes 99% of cycles.
```

### 14. Code Example: False Sharing (Cache Line Bouncing)
Two threads updating independent variables that happen to reside on the same 64-byte CPU cache line. They constantly invalidate each other's cache, destroying performance.
```cpp
#include <thread>
#include <vector>

struct Counters {
    int thread1_count;
    // int padding[16]; // Adding padding fixes the false sharing!
    int thread2_count;
};

Counters global_counters;

void work1() { for(int i=0; i<100000000; i++) global_counters.thread1_count++; }
void work2() { for(int i=0; i<100000000; i++) global_counters.thread2_count++; }

int main() {
    std::thread t1(work1); std::thread t2(work2);
    t1.join(); t2.join();
}
// Run: perf stat -e cache-misses ./app
// Will show massive L1 cache misses compared to the padded version.
```

### 15. Code Example: Branch Prediction Failure
Processing unsorted data causes the CPU's branch predictor to fail 50% of the time, flushing the execution pipeline.
```cpp
#include <vector>
#include <algorithm>
#include <random>

int processData(std::vector<int>& data) {
    int sum = 0;
    // If 'data' is sorted before this loop, it runs roughly 3x faster!
    // std::sort(data.begin(), data.end()); 
    for (int x : data) {
        if (x > 128) { // Unpredictable branch if data is random
            sum += x;
        }
    }
    return sum;
}

int main() {
    std::vector<int> data(1000000);
    // Fill with random data 0-255...
    processData(data);
}
// Run: perf stat -e branch-misses ./app
```

---

## Session 6: Debugging C++ Memory Models
### Introduction
C++11 introduced a formal memory model. While `std::mutex` provides sequential consistency (the easiest to reason about), lock-free programming requires understanding atomic operations and memory ordering. Debugging memory ordering bugs is notoriously difficult because they manifest as intermittent data races or logical inconsistencies on specific CPU architectures (like ARM).

### Important Points
- **Sequential Consistency (`std::memory_order_seq_cst`):** The default. All threads see all operations in the exact same order.
- **Acquire-Release Semantics:** Used to synchronize data between threads without a global order. An `acquire` operation ensures subsequent reads/writes aren't moved before it. A `release` operation ensures previous reads/writes aren't moved after it.
- **Relaxed Ordering (`std::memory_order_relaxed`):** No synchronization or ordering guarantees, only atomicity for the variable itself.

### 16. Code Example: Relaxed Ordering Data Race (Logical)
Using relaxed ordering can lead to threads seeing values out of order if not synchronized properly.
```cpp
#include <atomic>
#include <thread>
#include <iostream>

std::atomic<int> x(0);
std::atomic<int> y(0);
int r1 = 0, r2 = 0;

void thread1() {
    x.store(1, std::memory_order_relaxed);
    r1 = y.load(std::memory_order_relaxed);
}

void thread2() {
    y.store(1, std::memory_order_relaxed);
    r2 = x.load(std::memory_order_relaxed);
}

int main() {
    std::thread t1(thread1); std::thread t2(thread2);
    t1.join(); t2.join();
    // It is possible for both r1 == 0 AND r2 == 0 because 
    // the CPU/Compiler is free to reorder the load before the store!
    std::cout << "r1: " << r1 << ", r2: " << r2 << std::endl;
}
```

### 17. Code Example: Safe Acquire-Release
Correctly synchronizing a payload using acquire-release semantics.
```cpp
#include <atomic>
#include <thread>
#include <iostream>

std::atomic<bool> ready(false);
int payload = 0;

void producer() {
    payload = 42; // Normal memory write
    // Release ensures 'payload=42' happens before 'ready=true' becomes visible
    ready.store(true, std::memory_order_release); 
}

void consumer() {
    // Acquire ensures we don't read 'payload' until 'ready' is truly true
    while (!ready.load(std::memory_order_acquire)) {} 
    std::cout << "Payload: " << payload << std::endl; // Safely prints 42
}

int main() {
    std::thread t1(producer); std::thread t2(consumer);
    t1.join(); t2.join();
}
```

### External Links & Further Reading
- [C++ Memory Model Documentation (cppreference)](https://en.cppreference.com/w/cpp/atomic/memory_order)
- [GCC Wiki: Atomic Sync](https://gcc.gnu.org/wiki/Atomic/GCCMM/AtomicSync)
- [Preshing on Programming (Excellent blog on lock-free concurrency)](https://preshing.com/)
- [Valgrind DRD & Helgrind (Thread error detectors)](https://valgrind.org/docs/manual/drd-manual.html)

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

11. What is the primary difference between a Segmentation Fault and an Abort signal (SIGABRT)?
<details><summary>View Answer</summary>
<b>Answer:</b> Segfault is an invalid memory access; Abort is usually triggered intentionally by the program (e.g., glibc detecting a double free).
</details>

12. How can you find the specific line of code that triggered a crash inside GDB using a core dump?
<details><summary>View Answer</summary>
<b>Answer:</b> Run `gdb <binary> <core>`, then use the `bt` (backtrace) or `where` command.
</details>

13. Which command in GDB allows you to move up the call stack to inspect variables in the caller function?
<details><summary>View Answer</summary>
<b>Answer:</b> `up`
</details>

14. What does the `-Og` compiler flag do in GCC/Clang?
<details><summary>View Answer</summary>
<b>Answer:</b> It enables optimizations that do not interfere with debugging, providing a balance between performance and debuggability.
</details>

15. In AddressSanitizer (ASAN), what does the error `heap-buffer-overflow` signify?
<details><summary>View Answer</summary>
<b>Answer:</b> The program read from or wrote to memory outside the bounds of a dynamically allocated heap chunk.
</details>

16. Can Valgrind (Memcheck) detect out-of-bounds access on stack-allocated arrays?
<details><summary>View Answer</summary>
<b>Answer:</b> Usually no; Memcheck is primarily designed for heap allocations. ASAN is much better for stack bounds checking.
</details>

17. In GDB, how do you set a breakpoint that triggers only for a specific thread ID?
<details><summary>View Answer</summary>
<b>Answer:</b> `break <location> thread <thread_id>`
</details>

18. What is the purpose of the `thread apply all` command in GDB?
<details><summary>View Answer</summary>
<b>Answer:</b> It executes a specified GDB command (like `bt`) across all currently known threads.
</details>

19. Which Linux signal is sent by default when a program attempts to divide by zero?
<details><summary>View Answer</summary>
<b>Answer:</b> SIGFPE (Floating-Point Exception)
</details>

20. What information does the `nm` tool provide when run on a Linux binary?
<details><summary>View Answer</summary>
<b>Answer:</b> It lists the symbols (functions, global variables) stored in the object file.
</details>

21. What happens if you compile a C++ program without the `-g` flag and try to debug it with GDB?
<details><summary>View Answer</summary>
<b>Answer:</b> GDB will show assembly instructions and memory addresses, but won't map them back to variable names or source code lines.
</details>

22. Which command would you use to attach GDB to an already running process with PID 1234?
<details><summary>View Answer</summary>
<b>Answer:</b> `gdb -p 1234`
</details>

23. In `perf report`, what does a high percentage in the "Self" column indicate?
<details><summary>View Answer</summary>
<b>Answer:</b> The function itself is consuming those CPU cycles directly, not its children/callees.
</details>

24. What type of issue causes "False Sharing" in a multi-threaded application?
<details><summary>View Answer</summary>
<b>Answer:</b> Multiple threads modifying independent variables that happen to reside on the same CPU cache line, causing constant cache invalidations.
</details>

25. How does UndefinedBehaviorSanitizer (UBSAN) handle a detected issue by default?
<details><summary>View Answer</summary>
<b>Answer:</b> It prints an error message to standard error and continues execution, unlike ASAN which usually halts the program.
</details>

26. What does `std::memory_order_relaxed` guarantee in C++11 atomics?
<details><summary>View Answer</summary>
<b>Answer:</b> It guarantees atomicity of the operation on that specific variable, but enforces no synchronization or ordering constraints on surrounding memory operations.
</details>

27. Why are core dumps sometimes disabled by default on production Linux systems?
<details><summary>View Answer</summary>
<b>Answer:</b> To save disk space and to prevent leakage of sensitive memory data.
</details>

28. What is a "data race" in C++?
<details><summary>View Answer</summary>
<b>Answer:</b> When two or more threads concurrently access the same memory location, at least one access is a write, and the accesses are not synchronized.
</details>

29. In a CPU Flame Graph, what does the width of a box represent?
<details><summary>View Answer</summary>
<b>Answer:</b> The frequency/population of that function appearing in the sampled call stacks (i.e., how much CPU time it took).
</details>

30. When reverse debugging in GDB, what command initiates the recording of execution state?
<details><summary>View Answer</summary>
<b>Answer:</b> `record` (or `target record-full`)
</details>
