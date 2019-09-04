//! RBL Library

#![feature(lang_items)]
#![feature(global_asm)]
#![feature(asm)]
#![feature(panic_info_message)]
#![deny(warnings)]
#![no_std]

extern crate alloc;
extern crate lazy_static;
extern crate spin;
extern crate volatile;

#[macro_use]
extern crate bitflags;

use linked_list_allocator::LockedHeap;

#[macro_use]
mod serial;
mod boot;
mod consts;
mod device_tree;
mod lang_items;
mod memory;
mod trap;
mod load;

#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap = LockedHeap::empty();