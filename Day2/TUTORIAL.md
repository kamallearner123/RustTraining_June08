# Modern C++ Tutorial: The Trainer's Guide

Welcome to the deep dive. In this tutorial, we will explore the most impactful features of Modern C++ (spanning C++11 to C++23) using real-world scenarios. Our goal is to write code that is expressive, safe, and blazingly fast.

---

## 1. Modern Language Design Foundation

### 1.1 Resource Acquisition Is Initialization (RAII)
RAII is the bedrock of C++. In languages like Java or C#, you rely on a Garbage Collector. In C++, you rely on scope. The rule is simple: **Whoever acquires a resource must release it in their destructor.**

**Real-world scenario:** A database connection that must be closed even if an exception is thrown.

```cpp
#include <iostream>
#include <stdexcept>

class DBConnection {
public:
    DBConnection() { std::cout << "[DB] Connected.\n"; }
    ~DBConnection() { std::cout << "[DB] Disconnected (Cleaned up!).\n"; }
    void query() { throw std::runtime_error("Network failure!"); }
};

void processData() {
    DBConnection conn; // Resource acquired
    conn.query();      // Throws exception!
    // Destructor of 'conn' is GUARANTEED to be called here during stack unwinding.
}

int main() {
    try {
        processData();
    } catch (...) {
        std::cout << "Caught error.\n";
    }
    return 0;
}
```

### 1.2 Smart Pointers: Death to `new` and `delete`
Never use raw `new` or `delete`. 
- Use `std::unique_ptr` when you want strict, single ownership. It has exactly zero overhead compared to a raw pointer.
- Use `std::shared_ptr` when multiple entities need to keep an object alive (uses atomic reference counting).

```cpp
#include <memory>
#include <iostream>

struct SensorData {
    int temperature;
    SensorData(int t) : temperature(t) {}
    ~SensorData() { std::cout << "SensorData destroyed.\n"; }
};

void analyze() {
    // std::make_unique is safe and fast. No 'delete' needed.
    std::unique_ptr<SensorData> data = std::make_unique<SensorData>(42);
    
    // std::unique_ptr cannot be copied. It must be MOVED.
    std::unique_ptr<SensorData> transferred_data = std::move(data);
    
    if (!data) {
        std::cout << "Original pointer is now null.\n";
    }
} // transferred_data goes out of scope here -> SensorData is destroyed.
```

### 1.3 Move Semantics
Before C++11, returning a massive `std::vector` from a function meant copying every single element. Move semantics allow us to "steal" the internal pointers of temporary objects.

```cpp
#include <vector>
#include <iostream>

std::vector<int> generateHugeVector() {
    std::vector<int> vec(1000000, 42); // Allocate 1 million ints
    return vec; // Under the hood, this is MOVED, not copied! O(1) instead of O(N).
}

int main() {
    std::vector<int> my_data = generateHugeVector();
    std::cout << "Vector acquired instantly.\n";
}
```

---

## 2. C++20: The Game Changers

### 2.1 Concepts (Sane Template Errors)
Templates are incredibly powerful, but passing the wrong type used to result in 500 lines of incomprehensible compiler errors. **Concepts** allow you to document and enforce template constraints.

```cpp
#include <concepts>
#include <iostream>

// We define a constraint: T must be an integral type (int, long, etc.)
template <std::integral T>
T add(T a, T b) {
    return a + b;
}

int main() {
    std::cout << add(5, 10) << "\n"; // Works perfectly
    
    // add(5.5, 10.2); // COMPILER ERROR: constraints not satisfied (not an integral)
}
```

### 2.2 Ranges (Functional Pipelines)
Manipulating data with `<algorithm>` used to be verbose: `std::sort(vec.begin(), vec.end())`. C++20 Ranges allow us to chain operations lazily, just like in functional languages or Rust's iterators.

```cpp
#include <iostream>
#include <vector>
#include <ranges>

int main() {
    std::vector<int> logs = {200, 404, 500, 200, 403, 200};

    // We want to find all non-200 logs, format them, and print them.
    auto error_pipeline = logs 
        | std::views::filter([](int code) { return code != 200; })
        | std::views::transform([](int code) { return "Error: " + std::to_string(code); });

    // The operations are evaluated LAZILY here:
    for (const auto& err : error_pipeline) {
        std::cout << err << "\n";
    }
}
```

---

## 3. Concurrency and Coroutines

### 3.1 Async and Futures
Spawning a thread for a simple background calculation is overkill. `std::async` allows you to schedule tasks and retrieve their result asynchronously using a `std::future`.

```cpp
#include <iostream>
#include <future>
#include <chrono>

int calculateMeaningOfLife() {
    std::this_thread::sleep_for(std::chrono::seconds(2)); // Heavy work
    return 42;
}

int main() {
    std::cout << "Dispatching background task...\n";
    // Launch asynchronously
    std::future<int> result = std::async(std::launch::async, calculateMeaningOfLife);
    
    std::cout << "Doing other UI work...\n";
    
    // Wait for the background task to finish and get the result
    std::cout << "The answer is: " << result.get() << "\n";
}
```

### 3.2 C++20 Coroutines Overview
Coroutines are functions that can be suspended and resumed. They use the keywords `co_await`, `co_yield`, and `co_return`.
*Note: C++20 provides the low-level framework, but high-level types like `Task<T>` are typically implemented by libraries (like `cppcoro` or Rust's `tokio` equivalent).*

---

## 4. Modern Libraries: Don't Reinvent the Wheel

Modern C++ engineers leverage the open-source ecosystem.

### 4.1 `{fmt}` -> `std::format` (C++20)
Stop using `printf` (unsafe) and `std::cout` (slow and verbose). `std::format` (based on the `fmt` library) is the modern standard.

```cpp
#include <iostream>
#include <format> // C++20

int main() {
    std::string user = "Admin";
    int latency = 45;
    
    // Python-style f-strings in C++! Type-safe and incredibly fast.
    std::cout << std::format("User '{}' connected with {}ms latency.\n", user, latency);
}
```

### 4.2 Logging with `spdlog`
When building production systems, `std::cout` is unacceptable. You need asynchronous logging to files with log rotation. `spdlog` is the industry standard.
*(Example syntax assuming `spdlog` is linked)*:
```cpp
#include "spdlog/spdlog.h"
int main() {
    spdlog::info("Application started");
    spdlog::error("Connection dropped: IP {}", "192.168.1.1");
}
```

---

## 5. A Glimpse into C++23

### 5.1 `std::expected` (Error Handling Revolution)
Exceptions are heavy and disrupt control flow. Inspired by Rust's `Result<T, E>`, C++23 introduces `std::expected`. It forces the caller to acknowledge that a function might fail, without throwing an exception.

```cpp
#include <iostream>
#include <expected>
#include <string>

// Returns either a double (success) or a string (error)
std::expected<double, std::string> divide(double a, double b) {
    if (b == 0.0) {
        return std::unexpected("Divide by zero error!");
    }
    return a / b;
}

int main() {
    auto result = divide(10.0, 0.0);
    
    if (result) {
        std::cout << "Result: " << *result << "\n";
    } else {
        std::cout << "Failed: " << result.error() << "\n";
    }
}
```

---
### Conclusion
You have just reviewed the core techniques that separate legacy C++ codebases from modern, high-performance systems. In the lab exercises, you will apply these techniques to refactor old pointer-heavy code into memory-safe, expressive architectures.
