use crate::consts::PHYSICAL_MEMORY_OFFSET;
use crate::memory::frame_allocator::{alloc_frame, dealloc_frame, phys_to_virt};
use core::mem::ManuallyDrop;
pub use riscv::addr::*;
use riscv::asm::{sfence_vma, sfence_vma_all};
use riscv::paging::{FrameAllocator, FrameDeallocator};
use riscv::paging::{Mapper, PageTable, PageTableEntry, PageTableFlags as EF, Rv32PageTable};
use riscv::register::satp;

pub struct PageTableImpl {
    page_table: Rv32PageTable<'static>,
    root_frame: Frame,
    entry: Option<PageEntry>,
}

/// PageTableEntry: the contents of this entry.
/// Page: this entry is the pte of page `Page`.
pub struct PageEntry(&'static mut PageTableEntry, Page);

impl PageEntry {
    pub fn update(&mut self) {
        unsafe {
            sfence_vma(0, self.1.start_address().as_usize());
        }
    }

    pub fn accessed(&self) -> bool {
        self.0.flags().contains(EF::ACCESSED)
    }
    pub fn clear_accessed(&mut self) {
        self.0.flags_mut().remove(EF::ACCESSED);
    }

    pub fn dirty(&self) -> bool {
        self.0.flags().contains(EF::DIRTY)
    }
    pub fn clear_dirty(&mut self) {
        self.0.flags_mut().remove(EF::DIRTY);
    }

    pub fn writable(&self) -> bool {
        self.0.flags().contains(EF::WRITABLE)
    }
    pub fn set_writable(&mut self, value: bool) {
        self.0.flags_mut().set(EF::WRITABLE, value);
    }

    pub fn present(&self) -> bool {
        self.0.flags().contains(EF::VALID | EF::READABLE)
    }
    pub fn set_present(&mut self, value: bool) {
        self.0.flags_mut().set(EF::VALID | EF::READABLE, value);
    }

    pub fn target(&self) -> usize {
        self.0.addr().as_usize()
    }
    pub fn set_target(&mut self, target: usize) {
        let flags = self.0.flags();
        let frame = Frame::of_addr(PhysAddr::new(target));
        self.0.set(frame, flags);
    }

    pub fn user(&self) -> bool {
        self.0.flags().contains(EF::USER)
    }
    pub fn set_user(&mut self, value: bool) {
        self.0.flags_mut().set(EF::USER, value);
    }

    pub fn execute(&self) -> bool {
        self.0.flags().contains(EF::EXECUTABLE)
    }
    pub fn set_execute(&mut self, value: bool) {
        self.0.flags_mut().set(EF::EXECUTABLE, value);
    }
}

impl PageTableImpl {
    pub fn map(&mut self, addr: usize, target: usize) -> &mut PageEntry {
        // map the 4K `page` to the 4K `frame` with `flags`
        let flags = EF::VALID | EF::READABLE | EF::WRITABLE;
        let page = Page::of_addr(VirtAddr::new(addr));
        let frame = Frame::of_addr(PhysAddr::new(target));
        // we may need frame allocator to alloc frame for new page table(first/second)
        self.page_table
            .map_to(page, frame, flags, &mut FrameAllocatorForRiscv)
            .unwrap()
            .flush();
        self.get_entry(addr).expect("fail to get entry")
    }

    pub fn unmap(&mut self, addr: usize) {
        let page = Page::of_addr(VirtAddr::new(addr));
        let (_, flush) = self.page_table.unmap(page).unwrap();
        flush.flush();
    }

    pub fn get_entry(&mut self, vaddr: usize) -> Option<&mut PageEntry> {
        let page = Page::of_addr(VirtAddr::new(vaddr));
        if let Ok(e) = self.page_table.ref_entry(page.clone()) {
            let e = unsafe { &mut *(e as *mut PageTableEntry) };
            self.entry = Some(PageEntry(e, page));
            Some(self.entry.as_mut().unwrap())
        } else {
            None
        }
    }

    /// Create a new page table with kernel memory mapped
    pub fn new() -> Self {
        let mut pt = Self::new_bare();
        pt.map_kernel();
        pt
    }

    pub fn new_bare() -> Self {
        let frame = alloc_frame().expect("failed to allocate frame");

        let table =
            unsafe { &mut *(phys_to_virt(frame.start_address().as_usize()) as *mut PageTable) };
        table.zero();

        PageTableImpl {
            page_table: Rv32PageTable::new(table, PHYSICAL_MEMORY_OFFSET),
            root_frame: frame,
            entry: None,
        }
    }

    pub fn map_kernel(&mut self) {
        let table = unsafe {
            &mut *(phys_to_virt(self.root_frame.start_address().as_usize()) as *mut PageTable)
        };
        for i in 768..1024 {
            let flags =
                EF::VALID | EF::READABLE | EF::WRITABLE | EF::EXECUTABLE | EF::ACCESSED | EF::DIRTY;
            let frame = Frame::of_addr(PhysAddr::new((i << 22) - PHYSICAL_MEMORY_OFFSET));
            table[i].set(frame, flags);
        }
    }

    pub fn token(&self) -> usize {
        return self.root_frame.number() | (1 << 31);
    }

    pub unsafe fn set_token(token: usize) {
        asm!("csrw satp, $0" :: "r"(token) :: "volatile");
    }

    pub fn active_token() -> usize {
        let mut token: usize = 0;
        unsafe {
            asm!("csrr $0, satp" : "=r"(token) ::: "volatile");
        }
        token
    }

    pub fn flush_tlb() {
        unsafe {
            sfence_vma_all();
        }
    }

    /// Activate this page table
    pub unsafe fn activate(&self) {
        let old_token = Self::active_token();
        let new_token = self.token();
        println!("switch table {:x?} -> {:x?}", old_token, new_token);
        if old_token != new_token {
            Self::set_token(new_token);
            Self::flush_tlb();
        }
    }

    /// Execute function `f` with this page table activated
    pub unsafe fn with<T>(&self, f: impl FnOnce() -> T) -> T {
        let old_token = Self::active_token();
        let new_token = self.token();
        println!("switch table {:x?} -> {:x?}", old_token, new_token);
        if old_token != new_token {
            Self::set_token(new_token);
            Self::flush_tlb();
        }
        let ret = f();
        println!("switch table {:x?} -> {:x?}", new_token, old_token);
        if old_token != new_token {
            Self::set_token(old_token);
            Self::flush_tlb();
        }
        ret
    }
}

impl Drop for PageTableImpl {
    fn drop(&mut self) {
        dealloc_frame(self.root_frame);
    }
}

struct FrameAllocatorForRiscv;

impl FrameAllocator for FrameAllocatorForRiscv {
    fn alloc(&mut self) -> Option<Frame> {
        alloc_frame()
    }
}

impl FrameDeallocator for FrameAllocatorForRiscv {
    fn dealloc(&mut self, frame: Frame) {
        dealloc_frame(frame);
    }
}
