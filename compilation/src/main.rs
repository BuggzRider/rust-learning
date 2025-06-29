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

mod compilation_notes;
mod compiler_directives;

fn main() {
    println!("--- COMPILATION DEMO ---");
    compilation_notes::compilation_demo();

    println!("\n--- COMPILER DIRECTIVES DEMO ---");
    compiler_directives::compiler_directives_demo();
}
