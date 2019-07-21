use crate::interrupt::init as interrupt_init;

global_asm!(include_str!("boot/entry.asm"));

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    interrupt_init();
    crate::memory::init();
    crate::fs::init();
    crate::process::init();
    crate::clock::init();
    crate::process::kmain();
    loop {}
}
