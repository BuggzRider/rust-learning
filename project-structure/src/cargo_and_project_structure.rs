/*
========================================
Rust Learning Notes - Cargo & Project Structure
========================================

- cargo = Rust’s official build system + package manager.
- Create a new project:
    cargo new my_project
- Structure:
    my_project/
    ├── Cargo.toml
    └── src/
        └── main.rs
- Commands:
    cargo build or cargo b → compiles in debug mode
    cargo build --release → optimized for production
    cargo run or cargo r → builds and runs (default debug mode)
    cargo run --quiet → runs and suppresses build output
    cargo check → only type-checks to quickly find errors
    cargo fmt → formats all files in the project
    cargo clean → deletes build artifacts for a fresh build
*/

pub fn cargo_and_project_structure_demo() {
    println!("Cargo is Rust's build system and package manager!");
}
