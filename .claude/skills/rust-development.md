# Rust Development Skill


## Coding Principles

Always write idiomatic Rust.

Prefer:

- struct for data modeling
- trait for abstraction
- Result for error handling
- iterator for data processing


Avoid:

- unnecessary clone()
- excessive unwrap()
- unsafe code


## Code Modification Rules

Before modifying code:

Explain:

1. Target files
2. Reason
3. Expected impact


After modification:

Run:

cargo check

cargo test


## Architecture Rules

Respect existing module boundaries.

Do not move functionality between modules unless necessary.

Do not create new modules without explanation.