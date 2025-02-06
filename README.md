# Multiple Mutable References in Rust

This repository demonstrates a common error in Rust: creating multiple mutable references to the same variable.  This violates Rust's borrowing rules, which are designed to prevent data races and ensure memory safety.

The `bug.rs` file contains code that attempts to create multiple mutable references, resulting in a compile-time error. The `bugSolution.rs` file shows how to modify the code to avoid the error, demonstrating safe and correct ways to handle mutable references.

## Running the Code

1. Clone this repository.
2. Navigate to the directory.
3. Compile and run the code using `rustc bug.rs && ./bug` and `rustc bugSolution.rs && ./bugSolution`