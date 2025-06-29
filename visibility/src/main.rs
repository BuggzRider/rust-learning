/*
========================================
Rust Learning Notes - Visibility and Access Control
========================================

Keywords:
- pub — make the item public.
- self — refer to current module.
- super — refer to parent module.
- crate — refer to root of the crate.

Examples:
mod outer {
    pub mod inner {
        pub fn say_hello() {
            println!("Hi from inner!");
        }
    }

    pub fn call_inner() {
        // relative path
        self::inner::say_hello();
        // from parent
        super::helper();
    }
}

fn helper() {
    println!("I'm helper in root module.");
}

fn main() {
    outer::inner::say_hello();
    outer::call_inner();
    helper();
}
