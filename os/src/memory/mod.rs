pub mod frame_allocator;
mod paging;

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
    remap_kernel();
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

fn remap_kernel() {
    println!("remaping");
    let offset = KERNEL_OFFSET as usize - MEMORY_OFFSET as usize;
    use crate::memory::paging::{InactivePageTable, MemoryAttr};
    let mut pg_table = InactivePageTable::new(offset);
    pg_table.set(
        stext as usize,
        etext as usize,
        MemoryAttr::new().set_readonly().set_execute(),
    );
    pg_table.set(sdata as usize, edata as usize, MemoryAttr::new().set_WR());
    pg_table.set(
        srodata as usize,
        erodata as usize,
        MemoryAttr::new().set_readonly(),
    );
    pg_table.set(sbss as usize, ebss as usize, MemoryAttr::new().set_WR());
    pg_table.set(
        bootstack as usize,
        bootstacktop as usize,
        MemoryAttr::new().set_WR(),
    );
    unsafe {
        pg_table.activate();
    }
}
