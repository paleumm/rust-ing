# Ownership

C has explicit control over memory management, while Python uses a garbage collector. Rust, on the other hand, use the Ownership concepts.

#### Ownership Rules

- Each value in Rust has an owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.