# Data Types in Rust

- **To give data types to a variable use:**
  - `let x: i32 = 20;`
  - `let x = 20i32;`
- **Can add `_` to differentiate large numbers:**
  - `let x: i32 = 20_123;`
- **`usize` and `isize`** automatically pick the type based on the system architecture (e.g., 32-bit or 64-bit).
- **Escape Sequences:**
  - `\` escapes everything, `\n` for new line, `\"` for escaped double quote, etc.
  - If escaping multiple things in a string, use `r` before the string to get everything escaped:
    - `let filepath: &str = r"c:\mydoc\new\videos";`
- **Format Specifiers:**
  - You can add format specifiers to print formatted data.
  - Example:
    ```rust
    let pi: f64 = 3.14159256484135168484;
    println!("this current value of pi is {:.4}", pi); // 3.1416
    ```
