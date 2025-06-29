/*
========================================
VARIABLE SHADOWING IN RUST
========================================

- In Rust, you can re-declare a variable with the same name using 'let', even with a different type. This is called shadowing.
- Useful when you want to transform a value and reuse the same variable name with a new type.
- Shadowing is different from JS, where redeclaring a variable in the same scope is not allowed.
- Common use case: converting a string input to a number and reusing the same variable name.
*/

pub fn variable_shadowing_demo() {
    let gram_of_protein: &str = "100.35";
    println!("gram_of_protein as &str: {}", gram_of_protein);
    let gram_of_protein: f64 = 100.35;
    println!("gram_of_protein as f64: {}", gram_of_protein);
    let gram_of_protein: i32 = 100;
    println!("gram_of_protein as i32: {}", gram_of_protein);
}
