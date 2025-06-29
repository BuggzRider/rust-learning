/*
========================================
RUST ERRORS
========================================

- Rustc (the Rust compiler) provides detailed error messages in the terminal when your code fails to compile.
- Each error has a unique code (e.g., E0384) and a helpful explanation.
- You can get more info about an error code using:
    rustc --explain <ERROR_CODE>
  Example:
    rustc --explain E0384
- Official error index: https://doc.rust-lang.org/error_codes/error-index.html

Example: Triggering a common error (uncomment to see the error)
    let x = 5;
    x = 6; // ERROR: cannot assign twice to immutable variable (E0384)

To see the error explanation, run:
    rustc --explain E0384

Below is a working example that will not cause an error, but you can uncomment the error line to see the compiler's output.
*/

pub fn rust_errors_demo() {
    let mut x = 5;
    println!("x before: {}", x);
    x = 6; // This is fine because x is mutable
    println!("x after: {}", x);
    // let y = 10;
    // y = 20; // Uncomment to see E0384 error
}
