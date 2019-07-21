#![no_std]
#![no_main]

#[macro_use]
extern crate rust;

#[no_mangle]
pub fn main() {
    for i in 0..100 {
        println!("Hello, world!");
        for j in 0..1000 {
        }
    }
    println!("Hello, world!");
}