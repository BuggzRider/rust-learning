/*
========================================
RUST VARIABLES AND MUTABILITY
========================================

NAMING CONVENTIONS:
-----------------
1. Variables should use snake_case
   Example: my_variable, user_count, total_items

2. Unused variables should start with underscore
   Example: _unused_var

VARIABLE DECLARATIONS:
-------------------
- By default, variables are immutable (cannot be changed)
- Use 'mut' keyword to make them mutable
- If using a mutable variable and reassigning its value, the type must remain the same as initially declared
  Example:
    let mut count = 5; // count is i32
    count = 10;        // OK
    // count = "ten"; // ERROR: mismatched types

For variable shadowing, see variable_shadowing.rs for working examples.

SCOPE:
------
- Variables are only valid within the block they are declared in.
- Example below shows variable scope in action.

CONSTANTS:
----------
- Immutable variable and constant are different things.
- A constant is a name for a value that never changes.
- Constants can be declared anywhere, even at file level.
- Constants must have a type and a value known at compile time.
- Convention: use uppercase with underscores.
- Example:
    const TAX_RATE: f64 = 7.25;

TYPE ALIAS:
-----------
- A type alias is an alternative name for an existing type.
- Example:
    type Meters = i32;
    fn type_alias_example() {
        let mile_race_length: Meters = 1600;
        println!("Race length: {} meters", mile_race_length);
    }
*/

pub fn variables_demo() {
    // Immutable variable
    let apples = 5;
    println!("Immutable apples: {}", apples);
    // apples = 6; // ERROR: cannot assign twice to immutable variable

    // Mutable variable
    let mut oranges = 3;
    println!("Initial oranges: {}", oranges);
    oranges = 4; // OK, type is still i32
    println!("Updated oranges: {}", oranges);
    // oranges = "four"; // ERROR: expected integer, found &str

    // Unused variable example
    let _unused_fruits = 10;
    // The underscore prefix tells the compiler this is intentionally unused

    // Scope example
    let x = 5;
    {
        let y = 10;
        println!("x: {}, y: {}", x, y); // both x and y are accessible here
    }
    // println!("y: {}", y); // ERROR: y does not exist here
    println!("x: {}", x); // x is still accessible

    // Constant example
    const TAX_RATE: f64 = 7.25;
    println!("Constant TAX_RATE: {}", TAX_RATE);

    // Type alias example
    type Meters = i32;
    let mile_race_length: Meters = 1600;
    println!("Race length: {} meters", mile_race_length);
}
