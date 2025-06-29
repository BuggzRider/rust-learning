/*
========================================
Rust Learning Notes - Crates, Packages, and Modules
========================================

📦 Crate
- A crate is the smallest unit of compilation.
- Two types:
    - Binary Crate: has main.rs, produces an executable.
    - Library Crate: has lib.rs, reusable code without main().

📦 Package
- A package is a set of crates controlled by a Cargo.toml.
- Default layout:
    my_app/
    ├── Cargo.toml
    └── src/
        └── main.rs
- A package can have multiple crates using [[bin]] or workspaces.

📂 Modules
- Modules organize code inside a crate using mod keyword.
- Example:
    mod network;
    fn main() {
        network::connect();
    }
    // in network.rs
    pub fn connect() {
        println!("Connected!");
    }
*/

mod network;

pub fn crates_packages_modules_demo() {
    network::connect();
}
