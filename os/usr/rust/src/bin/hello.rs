#![no_std]
#![no_main]

#[macro_use]
extern crate rust;

#[no_mangle]
pub fn main() -> i32 {
    println!("Hello world!");
    return 0;
}