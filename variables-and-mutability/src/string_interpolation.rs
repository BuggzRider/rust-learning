/*
========================================
RUST STRING INTERPOLATION (println!)
========================================

You can print variables in Rust using the println! macro in several ways:

1. Basic interpolation with {}
2. Multiple variables in order
3. Positional arguments with numbers
4. Reusing variables in different positions
5. Named arguments (Rust 1.58+ feature)
6. New named arguments (Rust 1.58+)
*/

pub fn string_interpolation_demo() {
    let apples = 5;
    let oranges = 4;

    // 1. Basic interpolation
    println!("This year, my garden has {} apples", apples);

    // 2. Multiple variables
    println!("This year, my garden has {} apples and {} oranges", apples, oranges);

    // 3. Positional arguments
    println!("This year, my garden has {0} apples and {1} oranges", apples, oranges);

    // 4. Reusing variables
    println!("I have {0} apples. You have {1} oranges. Together we have {0} apples and {1} oranges!", apples, oranges);

    // 5. Named arguments (Rust 1.58+)
    println!("This year, my garden has {apples} apples and {oranges} oranges", apples=apples, oranges=oranges);

    // 6. New named arguments (Rust 1.58+)
    println!("This year, my garden has {apples} apples and {oranges} oranges");
}
