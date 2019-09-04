use xmas_elf::{ElfFile, program::Type};
use core::slice;

extern {
    fn _payload_start();
    fn _payload_end();
}

/// Return entry point
pub fn load_elf() -> usize {
    let data = unsafe {
        slice::from_raw_parts(_payload_start as usize as *const u8, _payload_end as usize - _payload_start as usize)
    };
    let elf = ElfFile::new(data).unwrap();
    for ph in elf.program_iter() {
        if ph.get_type() != Ok(Type::Load) {
            continue;
        }
        if ph.file_size() > 0 {
            let offset = ph.offset() as usize;
            let size = ph.file_size() as usize;
            let target = unsafe {
                slice::from_raw_parts_mut(ph.virtual_addr() as *mut u8, size)
            };
            let source = &elf.input[offset..offset+size];
            target.copy_from_slice(&source);
            println!("Copying elf to {:#X}", ph.virtual_addr());
        }
    }
    elf.header.pt2.entry_point() as usize
}

pub fn load(entry: usize, hartid: usize, dtb: usize) -> ! {
    println!("Entering to {:#X} with hart id {} and dtb at {:#X}", entry, hartid, dtb);
    unsafe {
        // delegate interrupts
        let interrupts = 1 << 1 | 1 << 5 | 1 << 9; // SSIP | STIP | SEIP
        asm!("csrw mideleg, $0": : "r"(interrupts) : : "volatile" );

        // delegate exception
        // instruction address misaligned
        // breakpoint
        // environment call from u-mode
        // instruction page fault
        // load page fault
        // store/amo page fault
        let exceptions = 1 << 0 | 1 << 3 | 1 << 8 | 1 << 12 | 1 << 13 | 1 << 15;
        asm!("csrw medeleg, $0": : "r"(exceptions) : : "volatile" );

        //mstatus::set_mie();
        // MSIE
        asm!("csrw mie, $0": : "r"(1<<3) : : "volatile" );

        //mstatus::set_mpie();
        //mstatus::set_mpp(mstatus::MPP::Supervisor);
        let mut mstatus: usize;
        asm!("csrr $0, mstatus": "=r"(mstatus) :  : : "volatile" );
        mstatus |= 1 << 13 | 1 << 14; // fs
        mstatus |= !(1 << 7); // mpie
        mstatus |= 1 << 11; // mpp = s
        asm!("csrw mstatus, $0": : "r"(mstatus) : : "volatile" );

        // set supervisor entry point
        asm!("csrw mepc, $0": : "r"(entry) : : "volatile" );

        // enter supervisor mode
        asm!("mret": : "{x10}"(hartid) "{x11}"(dtb) : : "volatile" );
    }
    loop {

    }
}