use super::attr::MemoryAttr;
use crate::memory::paging::PageTableImpl;
use alloc::boxed::Box;
use core::fmt::Debug;

pub trait MemoryHandler: Debug + 'static {
    fn box_clone(&self) -> Box<dyn MemoryHandler>;
    fn map(&self, pt: &mut PageTableImpl, addr: usize, attr: &MemoryAttr);
    fn unmap(&self, pt: &mut PageTableImpl, addr: usize);
}

impl Clone for Box<dyn MemoryHandler> {
    fn clone(&self) -> Box<dyn MemoryHandler> {
        self.box_clone()
    }
}

#[derive(Debug, Clone)]
pub struct Linear {
    offset: isize,
}

impl MemoryHandler for Linear {
    fn box_clone(&self) -> Box<dyn MemoryHandler> {
        Box::new(self.clone())
    }

    fn map(&self, pt: &mut PageTableImpl, addr: usize, attr: &MemoryAttr) {
        attr.apply(pt.map(addr, (addr as isize + self.offset) as usize));
    }

    fn unmap(&self, pt: &mut PageTableImpl, addr: usize) {
        pt.unmap(addr);
    }
}

impl Linear {
    pub fn new(off: isize) -> Self {
        Linear { offset: off }
    }
}

#[derive(Debug, Clone)]
pub struct ByFrame;

use crate::memory::frame_allocator::alloc_frame;
impl MemoryHandler for ByFrame {
    fn box_clone(&self) -> Box<dyn MemoryHandler> {
        Box::new(self.clone())
    }

    fn map(&self, pt: &mut PageTableImpl, addr: usize, attr: &MemoryAttr) {
        let target = alloc_frame()
            .expect("failed to allocate frame")
            .start_address()
            .as_usize();
        let entry = pt.map(addr, target);
        attr.apply(entry);
    }

    fn unmap(&self, pt: &mut PageTableImpl, addr: usize) {
        pt.unmap(addr);
    }
}

impl ByFrame {
    pub fn new() -> Self {
        ByFrame {}
    }
}
