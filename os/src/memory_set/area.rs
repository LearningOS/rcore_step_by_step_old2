use super::{attr::MemoryAttr, handler::MemoryHandler};
use crate::consts::PAGE_SIZE;
use crate::memory::paging::PageTableImpl;
use alloc::boxed::Box;

#[derive(Debug, Clone)]
pub struct MemoryArea {
    start: usize,
    end: usize,
    handler: Box<dyn MemoryHandler>,
    attr: MemoryAttr,
}

impl MemoryArea {
    pub fn map(&self, pt: &mut PageTableImpl) {
        for addr in page_range(self.start, self.end) {
            self.handler.map(pt, addr, &self.attr);
        }
    }

    fn unmap(&self, pt: &mut PageTableImpl) {
        for addr in page_range(self.start, self.end) {
            self.handler.unmap(pt, addr);
        }
    }

    pub fn is_overlap_with(&self, start_addr: usize, end_addr: usize) -> bool {
        let p1 = self.start / PAGE_SIZE;
        let p2 = (self.end - 1) / PAGE_SIZE + 1;
        let p3 = start_addr / PAGE_SIZE;
        let p4 = (end_addr - 1) / PAGE_SIZE + 1;
        !((p1 >= p4) || (p2 <= p3))
    }

    pub fn new(
        start_addr: usize,
        end_addr: usize,
        handler: Box<dyn MemoryHandler>,
        attr: MemoryAttr,
    ) -> Self {
        MemoryArea {
            start: start_addr,
            end: end_addr,
            handler: handler,
            attr: attr,
        }
    }
}

/// Return a range of page start address containing [start, end)
fn page_range(start: usize, end: usize) -> impl Iterator<Item = usize> {
    let start_page = start / PAGE_SIZE;
    let end_page = (end - 1) / PAGE_SIZE + 1;
    (start_page..end_page).map(|x| x * PAGE_SIZE)
}
