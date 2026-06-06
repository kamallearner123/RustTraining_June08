# Day 3: Rust Programming Fundamentals

## Overview
Day 3 introduces Rust, a systems programming language that guarantees memory safety without a garbage collector. We will cover the core philosophy of Rust, its unique ownership model, the expressive type system, robust error handling, and fearless concurrency.

---

## Session 1: Rust Philosophy
### Introduction
Rust aims to provide "Memory safety without GC" and "Zero-cost abstractions." The compiler acts as a strict pair-programmer, preventing entire classes of bugs (like segfaults and data races) at compile time.

### Important Points
- **Memory Safety:** Rust ensures that memory is automatically and safely managed. No dangling pointers, no double frees, no use-after-free.
- **Zero-cost Abstractions:** High-level features (like iterators and closures) compile down to low-level code that is as fast as if you wrote it by hand in C.
- **Cargo:** Rust's built-in package manager and build system. It simplifies dependency management significantly compared to CMake.

---

## Session 2: Ownership Model
### Introduction
The defining feature of Rust. We will deep dive into ownership rules, borrowing (references), lifetimes, and how Rust's move semantics differ fundamentally from C++.

### Important Points
- **Ownership Rules:** Each value has a single owner. When the owner goes out of scope, the value is dropped.
- **Borrowing:** You can create references to values (borrowing). You can have either *one mutable reference* (`&mut T`) OR *any number of immutable references* (`&T`), but never both at the same time. This prevents data races.
- **Move by Default:** Unlike C++, where assignment usually copies, in Rust, assigning a non-primitive value *moves* ownership.

### Code Example
```rust
fn main() {
    let s1 = String::from("hello");
    // Ownership is moved to s2. s1 is no longer valid.
    let s2 = s1; 
    // println!("{}", s1); // Compiler Error!

    let mut s3 = String::from("world");
    let r1 = &mut s3;
    // let r2 = &mut s3; // Compiler Error! Cannot borrow as mutable more than once at a time.
    r1.push_str("!");
}
```

---

## Session 3: Type System
### Introduction
Rust features a rich type system with Structs for data, Enums for algebraic data types (variants), Pattern Matching for control flow, and Traits for defining shared behavior (interfaces).

### Important Points
- **Enums & Pattern Matching:** Rust enums can hold data. The `match` statement is exhaustive, forcing the programmer to handle all possible cases, eliminating unhandled state bugs.
- **Traits:** Similar to interfaces in C++ or Java, but can be implemented for types outside of the type's definition.

### Code Example
```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => println!("Quitting"),
        Message::Move { x, y } => println!("Moving to {}, {}", x, y),
        Message::Write(text) => println!("Text: {}", text),
    } // Exhaustive: compiler ensures all variants are handled
}
```

---

## Session 4: Error Handling
### Introduction
Rust does not use exceptions. Instead, it uses algebraic data types (`Result` and `Option`) to handle recoverable errors, and `panic!` for unrecoverable errors.

### Important Points
- **Option<T>:** Represents an optional value: either `Some(T)` or `None`. Replaces null pointers.
- **Result<T, E>:** Represents a computation that may succeed (`Ok(T)`) or fail (`Err(E)`).
- **The `?` Operator:** Used to propagate errors up the call stack easily without verbose `match` boilerplate.

---

## Session 5: Concurrency in Rust
### Introduction
"Fearless Concurrency." Because of the ownership and borrowing rules, Rust prevents data races at compile time.

### Important Points
- **Send / Sync:** Marker traits. `Send` means ownership can be transferred across threads. `Sync` means it's safe to share references between threads.
- **Mutex & Arc:** `Mutex<T>` provides mutual exclusion. `Arc<T>` (Atomic Reference Counted) allows multiple threads to own the Mutex.

### Code Example
```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}
```

---

## Hands-on Lab
1. **Rewrite C++ to Rust:** Take a small C++ class managing a resource (provided in `lab3/cpp_module.cpp`) and rewrite it in Rust, observing how Rust handles the destructor (`Drop` trait).
2. **Fix the Borrow Checker:** You are given a Rust file (`lab3/buggy.rs`) that fails to compile due to ownership and borrowing violations. Fix the code to make the compiler happy.
3. **Thread-Safe Service:** Implement an in-memory Key-Value store struct. Wrap it in `Arc<RwLock<T>>` to allow multiple threads to read concurrently, but require exclusive access for writing. Spawn multiple reader and writer threads to test it.

---

## Multiple Choice Questions (Homework)

1. What guarantees does the Rust compiler make regarding memory safety?
<details><summary>View Answer</summary>
<b>Answer:</b> It prevents dangling pointers, null pointer dereferences, and data races at compile time without a Garbage Collector.
</details>

2. By default, what happens when you assign a non-primitive variable to another in Rust (e.g., `let a = String::from("x"); let b = a;`)?
<details><summary>View Answer</summary>
<b>Answer:</b> The value is moved, and the original variable (`a`) becomes invalid.
</details>

3. Which rule describes Rust's borrowing restrictions regarding mutable references?
<details><summary>View Answer</summary>
<b>Answer:</b> You can have exactly one mutable reference, OR any number of immutable references, but not both simultaneously.
</details>

4. What is the equivalent of a "Null Pointer" in safe Rust?
<details><summary>View Answer</summary>
<b>Answer:</b> There are no null pointers; Rust uses the `Option<T>` enum (`None` variant) to represent the absence of a value.
</details>

5. In Rust, what control flow operator forces you to exhaustively handle all possible variants of an Enum?
<details><summary>View Answer</summary>
<b>Answer:</b> `match`
</details>

6. What is the standard way to handle recoverable errors in Rust instead of throwing exceptions?
<details><summary>View Answer</summary>
<b>Answer:</b> Returning a `Result<T, E>` enum.
</details>

7. What does the `?` operator do when placed after a function call returning a `Result`?
<details><summary>View Answer</summary>
<b>Answer:</b> It unwrap the `Ok` value, or immediately returns from the current function propagating the `Err` value.
</details>

8. What do the `Send` and `Sync` traits signify in Rust?
<details><summary>View Answer</summary>
<b>Answer:</b> They are marker traits for concurrency. `Send` means a type can cross thread boundaries; `Sync` means a type can be safely shared by reference between threads.
</details>

9. When sharing state across multiple threads where mutation is required, which combination of standard library types is commonly used?
<details><summary>View Answer</summary>
<b>Answer:</b> `Arc<Mutex<T>>`
</details>

10. What is Cargo?
<details><summary>View Answer</summary>
<b>Answer:</b> Rust's official package manager and build system.
</details>

11. What is the standard library trait in Rust that handles the destruction and cleanup of resources?
<details><summary>View Answer</summary>
<b>Answer:</b> `Drop`
</details>

12. Can a struct implement the `Drop` trait and the `Copy` trait at the same time?
<details><summary>View Answer</summary>
<b>Answer:</b> No, `Copy` requires bitwise duplication, which is incompatible with custom destruction logic.
</details>

13. How do you explicitly denote the lifetime of a reference parameter in a Rust function signature?
<details><summary>View Answer</summary>
<b>Answer:</b> Using a tick followed by a lowercase name, like `&'a i32`.
</details>

14. What does the `static` lifetime mean?
<details><summary>View Answer</summary>
<b>Answer:</b> The reference points to data that lives for the entire duration of the program (like string literals).
</details>

15. What Rust keyword is used to implement a trait for a specific type?
<details><summary>View Answer</summary>
<b>Answer:</b> `impl` (e.g., `impl Trait for Type`)
</details>

16. What is the `unwrap()` method primarily used for?
<details><summary>View Answer</summary>
<b>Answer:</b> Extracting the inner value from an `Option` or `Result`, panicking immediately if it contains `None` or `Err`.
</details>

17. Which operator acts as shorthand for checking a `Result`, returning early with an error on `Err`, or extracting the value on `Ok`?
<details><summary>View Answer</summary>
<b>Answer:</b> The question mark operator (`?`)
</details>

18. What is the difference between `String` and `&str` in Rust?
<details><summary>View Answer</summary>
<b>Answer:</b> `String` is an owned, heap-allocated, growable string. `&str` is an immutable, borrowed string slice.
</details>

19. How do you declare a mutable variable in Rust?
<details><summary>View Answer</summary>
<b>Answer:</b> Using the `mut` keyword (e.g., `let mut x = 5;`). Variables are immutable by default.
</details>

20. What is a "Trait Object" in Rust?
<details><summary>View Answer</summary>
<b>Answer:</b> A way to achieve dynamic dispatch (polymorphism at runtime), using the `dyn Trait` syntax.
</details>

21. What macro is commonly used to format strings without printing them to standard output?
<details><summary>View Answer</summary>
<b>Answer:</b> `format!`
</details>

22. What does `RwLock` provide that `Mutex` does not?
<details><summary>View Answer</summary>
<b>Answer:</b> It allows multiple concurrent readers, only locking exclusively when a write is requested.
</details>

23. How do channels work for concurrency in Rust standard library?
<details><summary>View Answer</summary>
<b>Answer:</b> They allow threads to communicate by sending messages via a `Sender` (mpsc: multi-producer) to a `Receiver` (single-consumer).
</details>

24. What does the `move` keyword do when placed before a closure?
<details><summary>View Answer</summary>
<b>Answer:</b> It forces the closure to take ownership of the variables it captures from its environment.
</details>

25. What is the purpose of the `std::marker::PhantomData` type?
<details><summary>View Answer</summary>
<b>Answer:</b> It tells the compiler that a struct acts as though it stores a value of a certain type, even if it doesn't, usually to ensure proper drop checking or variance.
</details>

26. In Rust, what happens if an array index is out of bounds at runtime?
<details><summary>View Answer</summary>
<b>Answer:</b> The program immediately panics (halts safely), preventing a buffer overflow security vulnerability.
</details>

27. What is the `Cargo.toml` file used for?
<details><summary>View Answer</summary>
<b>Answer:</b> It is the manifest file for Cargo, used to specify project metadata and dependencies.
</details>

28. Which Rust feature replaces the concept of inheritance from object-oriented programming?
<details><summary>View Answer</summary>
<b>Answer:</b> Traits and Trait composition.
</details>

29. What is a generic type in Rust?
<details><summary>View Answer</summary>
<b>Answer:</b> A placeholder for a type that is resolved at compile time, allowing the same code to work with multiple types (similar to C++ templates).
</details>

30. Can you modify a variable that has an active immutable reference pointing to it?
<details><summary>View Answer</summary>
<b>Answer:</b> No, Rust enforces that while any immutable reference exists, the underlying value cannot be modified.
</details>
