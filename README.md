This repository demonstrates a common error in Rust: creating multiple mutable references to the same variable.  The code in `bug.rs` attempts to do this, leading to undefined behavior. The corrected version in `bugSolution.rs` shows how to avoid this issue using techniques like cloning or using interior mutability.