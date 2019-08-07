#![no_std]
#![no_main]

#[macro_use]
extern crate rust;

use rust::io::getc;

const LF: u8 = 0x0au8;
const CR: u8 = 0x0du8;

// IMPORTANT: Must define main() like this
#[no_mangle]
pub fn main() -> i32 {
    println!("Rust user shell");
    loop {
        let c = getc();
        match c {
            LF | CR => {
                print!("{}", LF as char);
                print!("{}", CR as char)
            }
            _ => print!("{}", c as char)
        }
    }
}