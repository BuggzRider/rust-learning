/*
========================================
RUST COMPILER DIRECTIVES (ATTRIBUTES)
========================================

- Compiler directives (attributes) are annotations that tell the compiler how to treat code.
- Syntax: Place above the item (line, function, or file) you want to affect.
- For a single line or function: #[attribute]
- For the whole file: #![attribute]

Common example: Suppressing warnings about unused variables
- #[allow(unused_variables)] // for next line or function
- #![allow(unused_variables)] // for the whole file
    - Note: Add an exclamation mark (!) after # if you want the directive to apply to the whole file, not just the next line or function. This tells Rust to apply the directive globally in the file, not just locally.
    - Example: Place `#![allow(unused_variables)]` at the very top of your file to suppress all unused variable warnings in that file.

Other common attributes:
- #[derive(Debug)] // automatically implements Debug trait
- #[test] // marks a function as a test
- #[allow(dead_code)] // suppresses warnings about unused code

Examples below:
*/

// Apply to the whole file (uncomment to suppress all unused variable warnings)
// #![allow(unused_variables)]

pub fn compiler_directives_demo() {
    // Apply to a single variable
    #[allow(unused_variables)]
    let temp = 42; // No warning for unused variable

    // Apply to a function
    #[allow(dead_code)]
    fn unused_function() {
        println!("This function is never called, but no warning!");
    }

    // Derive Debug for a struct
    #[derive(Debug)]
    struct User {
        name: String,
        age: u8,
    }
    let user = User { name: String::from("Alice"), age: 30 };
    println!("User struct with Debug: {:?}", user);
}
