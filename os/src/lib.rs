#![feature(lang_items)]
#![feature(asm)]
#![feature(panic_info_message)]
#![feature(global_asm)]
#![feature(alloc_error_handler)]
#![feature(alloc)]
#![no_std]

extern crate alloc;

#[macro_use]
pub mod io;

mod clock;
mod consts;
mod context;
mod init;
mod interrupt;
mod lang_items;
mod memory;

use buddy_system_allocator::LockedHeap;

#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap = LockedHeap::empty();
