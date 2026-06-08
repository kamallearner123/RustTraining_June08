# Debugging Playground

This directory contains standalone C programs designed to crash or behave improperly. These are perfect for practicing your debugging skills using `gdb`, `core` dumps, and `valgrind` / AddressSanitizer.

## How to use

First, build all the binaries. We have provided a `Makefile` that builds the code with `-g` (debug symbols) and `-O0` (no optimization) enabled.

```bash
make
```

This will produce three executables:
1. `invalid_memory.out` (Standard build)
2. `invalid_memory_asan.out` (Built with AddressSanitizer)
3. `crash_sim.out` (Standard build)

---

### Scenario 1: Invalid Memory Access
**Location:** `invalid_memory/main.c`

This program allocates a small buffer on the heap but writes far more data into it than it can hold (Heap Buffer Overflow), and then attempts to free the corrupted memory.

**Task A: Run without Sanitizer**
```bash
./invalid_memory.out
```
*Observe what happens. It might crash with a cryptic error like `free(): invalid next size (fast)`. This is glibc detecting heap corruption.*

**Task B: Run with AddressSanitizer**
```bash
./invalid_memory_asan.out
```
*Observe the difference. ASAN intercepts the exact line of code where the out-of-bounds write occurs and prints a detailed stack trace!*

---

### Scenario 2: Crash Simulation (Core Dumps & GDB)
**Location:** `crash_sim/main.c`

This program attempts to dereference a `NULL` pointer, causing an immediate `SIGSEGV` (Segmentation Fault).

**Task A: Analyze with GDB**
1. Run the program inside GDB:
   ```bash
   gdb ./crash_sim.out
   ```
2. Type `run` to start execution.
3. When it crashes, type `bt` (backtrace) to see the function call stack.
4. Type `print user` to inspect the pointer value (it will be `0x0`).

**Task B: Core Dump Analysis**
If core dumps are enabled on your system (`ulimit -c unlimited`), running `./crash_sim.out` normally will generate a `core` file. 
Load it into GDB to perform post-mortem analysis:
```bash
gdb ./crash_sim.out ./core
```
*(Or use `coredumpctl gdb` if you are on RHEL/systemd).*

---

### Cleaning Up
Run `make clean` to remove the compiled binaries and any generated core dumps.
