# Rust Immutable Reference to Mutable Reference Error

This repository demonstrates a common error in Rust where an immutable reference is created to a mutable reference. This leads to a compile-time error because the immutable reference prevents modification even if the underlying data is mutable. 

The `bug.rs` file contains the erroneous code, and `bugSolution.rs` shows how to correctly handle mutable references to avoid this error.