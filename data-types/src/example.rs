// Data Types Example in Rust

pub fn data_types_demo() {
    // Giving data types to variables
    let x: i32 = 20;
    let y = 20i32;
    println!("x: {}, y: {}", x, y);

    // Using _ to differentiate large numbers
    let big_number: i32 = 20_123;
    println!("big_number: {}", big_number);

    // usize and isize depend on system architecture
    let arr = [1, 2, 3, 4];
    let arr_len: usize = arr.len();
    println!("Array length (usize): {}", arr_len);

    // Escape sequences
    let text = "This is a line.\nThis is another line.\"Quoted\".";
    println!("{}", text);

    // Raw string for file paths
    let filepath: &str = r"c:\mydoc\new\videos";
    println!("Filepath: {}", filepath);

    // Format specifiers
    let pi: f64 = 3.14159256484135168484;
    println!("This current value of pi is {:.4}", pi); // 3.1416
}
