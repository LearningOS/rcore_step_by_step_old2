pub mod frame_allocator;
pub mod paging;

use self::frame_allocator::{init as init_frame_allocator, test as test_frame_allocator};
use crate::consts::*;
use crate::HEAP_ALLOCATOR;
use riscv::register::sstatus;

pub fn init() {
    unsafe {
        // Allow user memory access
        sstatus::set_sum();
    }
    init_heap();
    init_frame_allocator();
    test_frame_allocator();
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

extern "C" {
    // text
    fn stext();
    fn etext();
    // data
    fn sdata();
    fn edata();
    // read only
    fn srodata();
    fn erodata();
    // bss
    fn sbss();
    fn ebss();
    // kernel
    fn start();
    fn end();
    // boot
    fn bootstack();
    fn bootstacktop();
}
