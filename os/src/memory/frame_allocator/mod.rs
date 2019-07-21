use crate::consts::*;
use buddy_allocator::{log2_down, BuddyAllocator};
use lazy_static::*;
use riscv::addr::*;
use spin::Mutex;

// 物理页帧分配器
lazy_static! {
    pub static ref BUDDY_ALLOCATOR: Mutex<BuddyAllocator> = Mutex::new(BuddyAllocator::new());
}

/// Physical address of the first page after kernel end
static mut KERNEL_END: usize = 0;

pub fn init() {
    extern "C" {
        fn end();
    }
    let kernel_end = end as usize / PAGE_SIZE * PAGE_SIZE + PAGE_SIZE - KERNEL_OFFSET + MEMORY_OFFSET;
    unsafe {
        KERNEL_END = kernel_end;
    }
    let free_memory_pages = (MEMORY_END - kernel_end) / PAGE_SIZE;
    let mut bu = BUDDY_ALLOCATOR
        .lock()
        .init(log2_down(free_memory_pages) as u8);
    println!("++++init frame allocator succeed!++++");
}

use riscv::addr::*;

pub fn alloc_frame() -> Option<Frame> {
    alloc_frames(1)
}

pub fn alloc_frames(size: usize) -> Option<Frame> {
    unsafe {
        let ret = BUDDY_ALLOCATOR
            .lock()
            .alloc(size)
            .map(|id| id * PAGE_SIZE + KERNEL_END);
        ret.map(|addr| Frame::of_addr(PhysAddr::new(addr)))
    }
}

pub fn dealloc_frame(target: Frame) {
    dealloc_frames(target, 1);
}

pub fn dealloc_frames(target: Frame, size: usize) {
    unsafe {
        BUDDY_ALLOCATOR
            .lock()
            .dealloc(target.number() - KERNEL_END / PAGE_SIZE, size);
    }
}

pub fn test() {
    let frame1: Frame = alloc_frame().expect("failed to alloc frame");
    println!(
        "test frame_allocator: {:#x}",
        frame1.start_address().as_usize()
    );
    let frame2: Frame = alloc_frame().expect("failed to alloc frame");
    println!(
        "test frame_allocator: {:#x}",
        frame2.start_address().as_usize()
    );
    dealloc_frame(frame1);
    let frame3: Frame = alloc_frame().expect("failed to alloc frame");
    println!(
        "test frame_allocator: {:#x}",
        frame3.start_address().as_usize()
    );
    dealloc_frame(frame2);
    dealloc_frame(frame3);
}
