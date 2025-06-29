/*
========================================
RUST NOTES: VARIABLES, STRING INTERPOLATION, SHADOWING
========================================

This folder now contains multiple files for better organization:

1. variables.rs             // Variables and mutability
2. variable_shadowing.rs    // Variable shadowing
3. string_interpolation.rs  // String interpolation

To run the examples, call the functions from these files in main().

For Rust errors and compiler directives, see their respective folders.
*/

mod variables;
mod string_interpolation;
mod variable_shadowing;

fn main() {
    println!("--- VARIABLES AND MUTABILITY DEMO ---");
    variables::variables_demo();

    println!("\n--- VARIABLE SHADOWING DEMO ---");
    variable_shadowing::variable_shadowing_demo();

    println!("\n--- STRING INTERPOLATION DEMO ---");
    string_interpolation::string_interpolation_demo();
}
