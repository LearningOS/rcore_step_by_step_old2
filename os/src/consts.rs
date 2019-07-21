pub const KERNEL_OFFSET: usize = 0xC000_0000;

pub const MEMORY_OFFSET: usize = 0x8000_0000;

pub const MEMORY_END: usize = 0x8100_0000;

pub const PAGE_SIZE: usize = 0x1000;

pub const KERNEL_HEAP_SIZE: usize = 0x0010_0000;

pub const RECURSIVE_INDEX: usize = 0x3fd;

pub const PHYSICAL_MEMORY_OFFSET: usize = KERNEL_OFFSET - MEMORY_OFFSET;
