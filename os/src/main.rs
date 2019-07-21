#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(global_asm)]

use core::panic::PanicInfo;

#[macro_use]
pub mod io;

global_asm!(include_str!("boot/entry.asm"));

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    let a = "Hello";
    let b = "World";
    println!("{}, {}!", a, b);
    panic!("End of rust_main");
}

#[no_mangle]
pub extern fn abort() {
    panic!("abort!");
}