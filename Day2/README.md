# Day 2: Modern C++ (C++20 / C++23)

## Overview
Day 2 transitions from legacy C++ idioms to Modern C++ (C++20 and C++23). The focus is on safety, expressive language features, modern concurrency, and leveraging established open-source libraries to avoid reinventing the wheel.

---

## Session 1: Modern Language Design
### Introduction
We revisit core concepts that define modern C++: Resource Acquisition Is Initialization (RAII), the shift away from raw pointers to smart pointers, and understanding move semantics to avoid unnecessary copies.

### Important Points
- **RAII:** Bind the life cycle of a resource (memory, mutex, file handle) to the lifetime of an object. When the object goes out of scope, the destructor cleans up.
- **Smart Pointers:** `std::unique_ptr` for exclusive ownership (zero overhead). `std::shared_ptr` for shared ownership (reference counted). Never use `new`/`delete` directly.
- **Move Semantics:** `std::move` casts an object to an rvalue reference, allowing its resources to be "stolen" rather than copied, significantly boosting performance.

### Code Example
```cpp
#include <iostream>
#include <memory>
#include <vector>

class Resource {
public:
    Resource() { std::cout << "Acquired\n"; }
    ~Resource() { std::cout << "Released\n"; }
};

int main() {
    // RAII via smart pointers
    std::unique_ptr<Resource> res1 = std::make_unique<Resource>();
    
    // Move semantics
    std::unique_ptr<Resource> res2 = std::move(res1);
    // res1 is now null, res2 owns the resource.
    
    return 0; // Resource is automatically released here
}
```

---

## Session 2: C++20 Features
### Introduction
C++20 was a massive release. We will explore Concepts for template type safety, Ranges for functional-style data processing, and enhancements to `constexpr` for compile-time execution.

### Important Points
- **Concepts:** Replace obscure template errors with clear constraints. E.g., `template <std::integral T>`.
- **Ranges:** Composable, lazy views for algorithms. Replaces the clunky `std::algorithm(vec.begin(), vec.end())` pattern.
- **Constexpr:** More of the standard library is now `constexpr`, allowing complex computations to happen at compile-time, reducing runtime overhead.

### Code Example
```cpp
#include <iostream>
#include <vector>
#include <ranges>

// Concept constraint
template <std::integral T>
T add(T a, T b) { return a + b; }

int main() {
    std::vector<int> numbers = {1, 2, 3, 4, 5, 6};
    
    // Ranges: functional pipeline
    auto even_squares = numbers 
        | std::views::filter([](int n){ return n % 2 == 0; })
        | std::views::transform([](int n){ return n * n; });

    for (int n : even_squares) {
        std::cout << n << " "; // 4 16 36
    }
}
```

---

## Session 3: Concurrency & Coroutines
### Introduction
We cover the evolution of C++ concurrency from `std::thread` and `std::async` to the highly anticipated C++20 Coroutines for writing asynchronous code that looks synchronous.

### Important Points
- **Async/Futures:** `std::async` provides a high-level way to run tasks asynchronously and retrieve results via `std::future`.
- **Coroutines:** Functions that can suspend execution to be resumed later (`co_await`, `co_yield`, `co_return`). Note: C++20 provides the core mechanics, but the standard library lacks high-level coroutine types (like tasks), requiring libraries or custom wrappers.

---

## Session 4: Modern Libraries
### Introduction
Modern C++ development relies heavily on robust third-party libraries. We look at standard library improvements and industry standards like `fmt`, `spdlog`, `Boost`, and `googletest`.

### Important Points
- **fmt:** The foundation for C++20's `std::format`. Much safer and faster than `printf` or `iostreams`.
- **spdlog:** Extremely fast, header-only/compiled, C++ logging library.
- **googletest:** The defacto standard for unit testing and mocking in C++.

---

## Session 5: C++23 Highlights
### Introduction
A look into the immediate future with C++23 features like `std::expected` for error handling (a bridge to Rust's `Result`), improved ranges, and "deducing this".

### Important Points
- **std::expected:** A type that holds either an expected value or an error, eliminating the need for exceptions in error handling.
- **Deducing this:** Allows member functions to deduce the object they are called on, simplifying CRTP (Curiously Recurring Template Pattern) and reducing boilerplate.

---

## Hands-on Lab
1. **Refactor Legacy Code:** Take the provided `lab2/legacy_pointers.cpp` (which uses raw `new`/`delete` and suffers from leaks) and refactor it completely using `std::unique_ptr` and `std::shared_ptr`.
2. **Ranges Pipeline:** Implement a data processing pipeline using C++20 Ranges that reads a list of structured data, filters out invalid entries, transforms the valid ones, and aggregates the result.
3. **Async Workflow:** Replace a blocking network mock function with an asynchronous coroutine-based workflow using C++20 coroutine primitives (or a provided simple task wrapper).

---

## Multiple Choice Questions (Homework)

1. What is the core principle of RAII in C++?
<details><summary>View Answer</summary>
<b>Answer:</b> Tying resource management (acquisition and release) to object lifetime (constructor and destructor).
</details>

2. Which smart pointer provides exclusive, zero-overhead ownership of a dynamically allocated object?
<details><summary>View Answer</summary>
<b>Answer:</b> `std::unique_ptr`
</details>

3. What does `std::move` actually do under the hood?
<details><summary>View Answer</summary>
<b>Answer:</b> It casts its argument to an rvalue reference, enabling move constructors/assignments.
</details>

4. In C++20, what feature is used to apply semantic constraints to template arguments to produce better compiler errors?
<details><summary>View Answer</summary>
<b>Answer:</b> Concepts
</details>

5. What is a key characteristic of C++20 Range views?
<details><summary>View Answer</summary>
<b>Answer:</b> They are evaluated lazily (computation happens only when iterating).
</details>

6. Which C++20 keywords are used within a function to make it a coroutine?
<details><summary>View Answer</summary>
<b>Answer:</b> `co_await`, `co_yield`, and `co_return`.
</details>

7. Which modern C++ library heavily inspired the C++20 `std::format` feature?
<details><summary>View Answer</summary>
<b>Answer:</b> `fmt`
</details>

8. What is the primary purpose of `spdlog` in modern C++ projects?
<details><summary>View Answer</summary>
<b>Answer:</b> Extremely fast, asynchronous and synchronous application logging.
</details>

9. In C++23, what is `std::expected` primarily used for?
<details><summary>View Answer</summary>
<b>Answer:</b> Returning either a successful value or an error object without throwing exceptions.
</details>

10. What C++23 feature simplifies writing member functions that need to know if the object is const or an rvalue reference?
<details><summary>View Answer</summary>
<b>Answer:</b> Deducing `this`
</details>

11. What problem does `std::shared_ptr` solve that `std::unique_ptr` cannot?
<details><summary>View Answer</summary>
<b>Answer:</b> Allowing multiple owners of the same dynamically allocated object using reference counting.
</details>

12. In C++20 Concepts, what keyword is used to define a new concept?
<details><summary>View Answer</summary>
<b>Answer:</b> `concept`
</details>

13. Which standard header must be included to use C++20 Ranges?
<details><summary>View Answer</summary>
<b>Answer:</b> `<ranges>`
</details>

14. How do C++20 Views differ from traditional containers?
<details><summary>View Answer</summary>
<b>Answer:</b> Views are non-owning and evaluated lazily, whereas containers own their data.
</details>

15. What is the return type of a function that uses `co_yield` or `co_await`?
<details><summary>View Answer</summary>
<b>Answer:</b> It must return a coroutine-compatible type, not a standard type like `int`.
</details>

16. What is a "Promise" in the context of C++20 Coroutines?
<details><summary>View Answer</summary>
<b>Answer:</b> The object that controls the coroutine's state and is used to return a value or exception.
</details>

17. Why is `std::format` preferred over `std::cout` (iostreams) for complex output?
<details><summary>View Answer</summary>
<b>Answer:</b> It is faster, type-safe, and allows for much clearer formatting strings (similar to Python's f-strings).
</details>

18. What C++23 feature allows you to elegantly handle both values and errors without exceptions?
<details><summary>View Answer</summary>
<b>Answer:</b> `std::expected`
</details>

19. In C++, what does the "Rule of Zero" state?
<details><summary>View Answer</summary>
<b>Answer:</b> A class should avoid defining any custom destructor, copy/move constructors, or assignment operators by relying on RAII members (like smart pointers).
</details>

20. What is a "Dangling Pointer"?
<details><summary>View Answer</summary>
<b>Answer:</b> A pointer that still points to a memory location after the object it pointed to has been deleted or deallocated.
</details>

21. Which function is used to create a `std::shared_ptr` efficiently in one allocation?
<details><summary>View Answer</summary>
<b>Answer:</b> `std::make_shared`
</details>

22. What happens to a `std::unique_ptr` when it is passed by value to a function?
<details><summary>View Answer</summary>
<b>Answer:</b> It causes a compile error, as `std::unique_ptr` cannot be copied; it must be moved.
</details>

23. What C++ feature eliminates the need to specify types explicitly when they can be inferred from the initializer?
<details><summary>View Answer</summary>
<b>Answer:</b> `auto`
</details>

24. How does `constexpr` differ from `const`?
<details><summary>View Answer</summary>
<b>Answer:</b> `constexpr` implies the value or function *can* be evaluated at compile-time, while `const` just means the value cannot be mutated after initialization.
</details>

25. What is the syntax for a lambda expression that captures all variables by reference?
<details><summary>View Answer</summary>
<b>Answer:</b> `[&]`
</details>

26. Which C++20 feature allows you to initialize variables inside a range-based for loop?
<details><summary>View Answer</summary>
<b>Answer:</b> init-statement in range-based for (e.g., `for (int x = 0; auto i : vec)`)
</details>

27. What is the primary advantage of the Curiously Recurring Template Pattern (CRTP)?
<details><summary>View Answer</summary>
<b>Answer:</b> Static polymorphism (polymorphism at compile-time without virtual function overhead).
</details>

28. Which C++23 feature aims to simplify CRTP?
<details><summary>View Answer</summary>
<b>Answer:</b> Deducing `this`
</details>

29. What is a `std::weak_ptr`?
<details><summary>View Answer</summary>
<b>Answer:</b> A smart pointer that observes a `std::shared_ptr` without increasing the reference count, preventing circular references.
</details>

30. In the context of `spdlog`, what does "asynchronous logging" mean?
<details><summary>View Answer</summary>
<b>Answer:</b> Log messages are pushed to a queue and written to disk by a background thread, preventing the main thread from blocking on I/O.
</details>
