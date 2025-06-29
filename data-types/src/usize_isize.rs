// =====================================
// DATA TYPES: usize and isize
// =====================================
//
// usize and isize automatically pick the type based on the system architecture (e.g., 32-bit or 64-bit).
//
// Example:
// let arr = [1, 2, 3, 4];
// let arr_len: usize = arr.len();

fn main() {
    let arr = [1, 2, 3, 4];
    let arr_len: usize = arr.len();
    println!("Array length (usize): {}", arr_len);
}
