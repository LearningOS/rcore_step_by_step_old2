use crate::interrupt::init as interrupt_init;
use crate::clock::init as clock_init;
use crate::memory::init as memory_init;
use crate::consts::*;
use crate::process::init as process_init;

global_asm!(include_str!("boot/entry.asm"));

#[no_mangle]
pub extern "C" fn rust_main(hartid: usize, dtb: usize) -> ! {
    interrupt_init();
    println!("Hello RISCV ! in hartid {}, dtb @ {:#x} ", hartid, dtb);
    memory_init(dtb);
    clock_init();
    process_init();
    loop {}
}	
