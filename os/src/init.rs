use crate::interrupt::init as interrupt_init;
use crate::clock::init as clock_init;
use crate::memory::init as memory_init;

global_asm!(include_str!("boot/entry.asm"));

#[no_mangle]
pub extern "C" fn rust_main(hartid: usize, dtb: usize) -> ! {
    interrupt_init();
    println!("Hello RISCV ! in hartid {}, dtb @ {:#x} ", hartid, dtb);
    memory_init(dtb);
    clock_init();
	//from asm code, we know addr 0xc0020020 has content 0x12112623
	let x_ptr: *mut u32 = 0xc0020020 as *mut u32;
	unsafe{ 
      let x = *x_ptr;
      println!("addr:{:?}: content:0x{:x} ",x_ptr, x);
    }
    loop {}
}	
