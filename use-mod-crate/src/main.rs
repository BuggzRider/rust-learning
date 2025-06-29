/*
========================================
Rust Learning Notes - use, mod, crate Mechanics
========================================

- mod xyz; → Tells compiler to include a module file xyz.rs or xyz/mod.rs
- use → Bring items into scope
- crate:: → Start from root of the crate

Example:
// lib.rs or main.rs
mod utils;

use crate::utils::math::add;

fn main() {
    println!("{}", add(2, 3));
}
// utils/math.rs
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
*/

mod utils {
    pub mod math {
        pub fn add(a: i32, b: i32) -> i32 {
            a + b
        }
    }
}

use crate::utils::math::add;

fn main() {
    println!("2 + 3 = {}", add(2, 3));
}
