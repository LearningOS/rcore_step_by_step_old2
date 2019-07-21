use riscv::register::sstatus;
use crate::consts::*;
use crate::HEAP_ALLOCATOR;

pub fn init(dtb: usize) {
    unsafe {
        // Allow user memory access
        sstatus::set_sum();
    }
	init_heap();
}

fn init_heap() {
    static mut HEAP: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];
    unsafe {
        HEAP_ALLOCATOR
            .lock()
            .init(HEAP.as_ptr() as usize, KERNEL_HEAP_SIZE);
    }
    println!("heap init end");
}

pub enum PageFault{
    LoadPageFault,
    StorePageFault,
}

use crate::context::TrapFrame;
pub fn do_pgfault(tf: &mut TrapFrame, style: PageFault) {
    match style {
        PageFault::LoadPageFault => panic!("load pagefault"),
        PageFault::StorePageFault => panic!("store pagefault"),
    }
}

#[alloc_error_handler]
fn foo(_: core::alloc::Layout) -> ! {
    panic!("DO NOTHING alloc_error_handler set by kernel");
}
