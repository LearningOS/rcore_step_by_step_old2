//! RBL Library

#![feature(lang_items)]
#![feature(alloc)]
#![feature(global_asm)]
#![feature(panic_info_message)]
#![no_std]

extern crate alloc;

use linked_list_allocator::LockedHeap;

mod lang_items;
mod boot;
mod memory;
mod device_tree;
mod consts;
mod trap;

#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap = LockedHeap::empty();