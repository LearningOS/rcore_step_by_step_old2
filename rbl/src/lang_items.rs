// Rust language items

use core::panic::PanicInfo;
use core::alloc::Layout;

#[lang = "eh_personality"] 
extern fn eh_personality() {
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {

    }
}

#[lang = "oom"]
fn oom(_: Layout) -> ! {
    panic!("out of memory");
}
