// =====================================
// DATA TYPES: Raw Strings
// =====================================
//
// If escaping multiple things in a string, use r before the string to get everything escaped.
// Example:
// let filepath: &str = r"c:\mydoc\new\videos";
//
fn main() {
    let filepath: &str = r"c:\mydoc\new\videos";
    println!("Filepath: {}", filepath);
}
