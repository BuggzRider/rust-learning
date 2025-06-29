// ...existing code from compilation/src/main.rs...
/*
========================================
Rust Learning Notes - Compilation & Binaries
========================================

- Rust uses rustc to compile source code into native machine code.
- Compilation targets are platform-specific:
    Windows → .exe
    Linux → ELF binary
    macOS → Mach-O binary
- You can cross-compile using:
    rustup target add x86_64-pc-windows-gnu
    cargo build --target x86_64-pc-windows-gnu
- Similar to C/C++ where you must specify target triple for cross-platform support.
*/

pub fn compilation_demo() {
    println!("Rust compiles to platform-dependent binaries!");
}
