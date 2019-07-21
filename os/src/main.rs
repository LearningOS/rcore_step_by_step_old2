#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(global_asm)]

use core::panic::PanicInfo;
pub mod io;

global_asm!(include_str!("boot/entry.asm"));

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    io::puts("666666");
    loop {}
}

#[no_mangle]
pub extern "C" fn abort() {
    panic!("abort!");
}
