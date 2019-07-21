#![no_std]
#![no_main]

#[macro_use]
extern crate rust;

// IMPORTANT: Must define main() like this
#[no_mangle]
pub fn main() -> i32 {
    println!("Rust user shell");
    loop {}
}