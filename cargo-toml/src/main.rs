/*
========================================
Rust Learning Notes - Cargo.toml & TOML
========================================

- Cargo.toml = manifest file for metadata & dependencies.
- Written in TOML (Tom’s Obvious, Minimal Language).
    [package]
    name = "hello_world"
    version = "0.1.0"
    edition = "2021"

    [dependencies]
    rand = "0.8"
- Strongly typed: supports arrays, booleans, strings, integers.
- Similar to package.json, but stricter.
*/

fn main() {
    println!("Cargo.toml is the manifest file for Rust projects, written in TOML.");
}
