// =====================================
// DATA TYPES: Escape Sequences
// =====================================
//
// \ escapes everything, \n new line, \" escaped double quote, etc.
// If escaping multiple things in a string, use r before the string to get everything escaped.
// Example:
// let filepath: &str = r"c:\mydoc\new\videos";

fn main() {
    let text = "This is a line.\nThis is another line.\"Quoted\".";
    println!("{}", text);
}
