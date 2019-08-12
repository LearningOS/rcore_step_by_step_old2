#![no_std]
#![no_main]
#![feature(alloc)]

extern crate alloc;

#[macro_use]
extern crate rust;

use rust::io::getc;
use rust::syscall::sys_exec;
use alloc::string::String;

const LF: u8 = 0x0au8;
const CR: u8 = 0x0du8;

// IMPORTANT: Must define main() like this
#[no_mangle]
pub fn main() -> i32 {
    println!("Rust user shell");
    let mut line: String = String::new();
    print!(">> ");
    loop {
        let c = getc();
        match c {
            LF | CR => {
                println!("");
                if !line.is_empty() {
                    sys_exec(line.as_ptr());
                    line.clear();
                }
                print!(">> ");
            }
            _ => {
                print!("{}", c as char);
                line.push(c as char)
            }
        }
    }
}