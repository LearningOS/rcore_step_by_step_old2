//! Boot init code

use super::memory;
use super::load;
use super::device_tree;

#[no_mangle]
pub extern fn abort() {
    panic!("abort");
}

#[no_mangle]
pub extern fn boot_first_hart(hartid: usize, dtb: usize) -> ! {
    memory::clear_bss();
    memory::init_heap();
    device_tree::init(dtb);

    // load payload elf
    let entry = load::load_elf();

    // enter supervisor mode
    load::load(entry, hartid, dtb);
}
global_asm!(include_str!("payload.S"));
global_asm!(include_str!("trap.S"));
global_asm!(include_str!("boot.S"));