use crate::clock::init as clock_init;
use crate::interrupt::init as interrupt_init;

global_asm!(include_str!("boot/entry.asm"));

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    interrupt_init();
    clock_init();
    test_page_table();
    loop {}
}

fn test_page_table() {
    // test read
    let ptr = 0xc0400000 as *const u32;
    let value = unsafe { ptr.read() };
    println!("addr: {:?}, value: {:#x}", ptr, value);

    // test write: page fault!
    unsafe {
        (0xc0000000 as *mut u32).write(0);
    }
}
