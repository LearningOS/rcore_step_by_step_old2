use core::mem::size_of;
use crate::consts::KERNEL_HEAP_SIZE;
use crate::HEAP_ALLOCATOR;

extern {
    fn sbss();
    fn ebss();
}

pub fn clear_bss() {
    unsafe {
        let start = sbss as usize;
        let end = ebss as usize;
        let step = size_of::<usize>();
        for i in (start..end).step_by(step) {
            (i as *mut usize).write(0);
        }
    }
}

pub fn init_heap() {
    static mut HEAP: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];
    unsafe {
        HEAP_ALLOCATOR.lock().init(HEAP.as_ptr() as usize, KERNEL_HEAP_SIZE);
    }
}