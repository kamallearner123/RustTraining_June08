# Introduction to Modern C++ (C++20 / C++23)

Welcome to Day 2! Today, we leave behind the manual memory management and legacy paradigms of C and older C++ standards, stepping into the era of **Modern C++**.

## The Evolution of C++

To truly appreciate Modern C++, you have to understand where we came from:

1. **C and C++98/03 (The Dark Ages):**
   - Manual memory management (`new` and `delete`).
   - Heavy use of raw pointers.
   - Verbose and confusing template error messages.
   - "C with Classes" mindset.

2. **C++11/14 (The Renaissance):**
   - The birth of "Modern C++".
   - Introduction of **Move Semantics** (the biggest performance boost in C++ history).
   - **Smart Pointers** (`std::unique_ptr`, `std::shared_ptr`) practically eliminated the need for `delete`.
   - `auto`, lambdas, and type traits made code infinitely more readable.

3. **C++17 (The Polish):**
   - `std::optional`, `std::variant`, and `std::any`.
   - Structured bindings and `if constexpr`.
   - Parallel algorithms.

4. **C++20/23 (The Modern Era - Today's Focus):**
   - **Concepts:** Finally, sane template error messages and semantic requirements.
   - **Ranges:** Functional-style pipelines for data processing.
   - **Coroutines:** Asynchronous programming that looks synchronous.
   - **std::expected (C++23):** Error handling without the heavy runtime cost of exceptions.

## The Modern C++ Philosophy

As a professional systems engineer, your mindset must shift. The Modern C++ philosophy is built on three pillars:

1. **Zero-Cost Abstractions:** What you don't use, you don't pay for. What you do use, you couldn't hand-code any better. We use high-level syntax that compiles down to the tightest possible assembly.
2. **Safety by Default:** Raw pointers are for viewing, never for owning. RAII (Resource Acquisition Is Initialization) is non-negotiable. If a resource is acquired, its release must be guaranteed by a destructor.
3. **Expressiveness:** Code is read ten times more than it is written. Features like Ranges and Concepts allow you to write code that declares *what* it does, not *how* it does it.

Prepare to unlearn some legacy habits. By the end of today, you will be writing C++ that is safer, faster, and more beautiful than ever before. Let's dive into the **[TUTORIAL.md](TUTORIAL.md)**!
