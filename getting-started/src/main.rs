/*
========================================
🦀 Rust Learning Notes - Getting Started
========================================

- main() is the entry point of every Rust executable.
- Rust is compiled and statically typed (like C).
- Compilation output is platform-dependent, just like C.
- Compile manually using:
    rustc file.rs
    ./file    // or file.exe on Windows
- rustfmt formats a single file.
- Use println!() to print — it’s a macro, not a function (hence the !).
    - Macros are like functions, but more powerful and processed before compilation.
*/

fn main() {
    println!("Hello, Rust beginner!");
    println!("This is the entry point of every Rust executable.");
}
