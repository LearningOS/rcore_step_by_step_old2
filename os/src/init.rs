use crate::interrupt::init as interrupt_init;
use crate::clock::init as clock_init;
use crate::memory::init as memory_init;
use crate::consts::*;
use crate::process::{ init as process_init, kmain };
use crate::fs::init as fs_init;

global_asm!(include_str!("boot/entry.asm"));

#[no_mangle]
pub extern "C" fn rust_main(hartid: usize, dtb: usize) -> ! {
    interrupt_init();
    println!("Hello RISCV ! in hartid {}, dtb @ {:#x} ", hartid, dtb);
    memory_init(dtb);
    fs_init();
    clock_init();
    process_init();
    kmain();
    loop {}
}

global_asm!(concat!(
    r#"
	.section .data
	.global _user_img_start
	.global _user_img_end
_user_img_start:
    .incbin ""#,
    env!("SFSIMG"),
    r#""
_user_img_end:
"#
)); 
