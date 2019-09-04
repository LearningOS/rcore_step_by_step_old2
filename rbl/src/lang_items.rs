// Rust language items

use core::panic::PanicInfo;
use core::fmt::Write;
use core::alloc::Layout;
use alloc::string::String;

#[lang = "eh_personality"] 
extern fn eh_personality() {
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let location = info.location().unwrap();
    let message = info.message().unwrap();
    let mut output = String::new();
    write!(&mut output, "panic info {}", message);

    loop {

    }
}

#[lang = "oom"]
fn oom(_: Layout) -> ! {
    panic!("out of memory");
}
