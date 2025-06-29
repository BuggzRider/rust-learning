// ================================
// RUST NOTES: DATA TYPES EXAMPLES
// ================================
//
// This folder contains multiple files for better organization:
//
// 1. datatype.rs         // Assigning data types to variables
// 2. large_number.rs     // Using _ for large numbers
// 3. usize_isize.rs      // System-dependent integer types
// 4. escape_sequences.rs // Escape sequences in strings
// 5. raw_string.rs       // Raw strings for file paths
// 6. format_specifier.rs // Format specifiers in printing
//
// To run the examples, use: cargo run --bin <filename> (after adding [[bin]] section if needed)
//

mod example;

fn main() {
    println!("--- DATA TYPES EXAMPLES ---");
    example::data_types_demo();
}
