use crate::clock::{clock_set_next_event, TICK};
use crate::context::TrapFrame;
use crate::process::tick;
use riscv::register::{mcause::*, sscratch, sstatus, stvec};

#[no_mangle]
pub fn init() {
    extern "C" {
        fn __alltraps();
    }
    unsafe {
        sscratch::write(0);
        sstatus::set_sie();
        stvec::write(__alltraps as usize, stvec::TrapMode::Direct);
    }
    println!("++++setup interrupt !++++");
}

global_asm!(include_str!("trap/trap.asm"));

#[no_mangle]
pub extern "C" fn rust_trap(tf: &mut TrapFrame) {
    match tf.scause.cause() {
        Trap::Exception(Exception::Breakpoint) => breakpoint(),
        Trap::Interrupt(Interrupt::SupervisorTimer) => timer(),
        Trap::Exception(Exception::InstructionPageFault) => page_fault(tf),
        Trap::Exception(Exception::LoadPageFault) => page_fault(tf),
        Trap::Exception(Exception::StorePageFault) => page_fault(tf),
        _ => panic!("unexpected trap: {:#x?}", tf),
    }
}

fn breakpoint() {
    panic!("a breakpoint set by kernel");
}

fn timer() {
    // 响应当前时钟中断的同时，手动设置下一个时钟中断。这一函数调用同时清除了 MTIP ，使得 CPU 知道当前这个中断被正确处理。
    clock_set_next_event();
    unsafe {
        TICK = TICK + 1;
        if TICK % 100 == 0 {
            println!("100 ticks!");
        }
    }
    tick();
}

fn page_fault(tf: &mut TrapFrame) {
    println!("{:?} @ {:#x}", tf.scause.cause(), tf.stval);
    panic!("page fault");
}

#[inline(always)]
pub fn enable_and_wfi() {
    // 使能中断并等待中断
    unsafe {
        asm!("csrsi sstatus, 1 << 1; wfi" :::: "volatile");
    }
}

#[inline(always)]
pub fn disable_and_store() -> usize {
    // 禁用中断并返回当前中断状态
    let sstatus: usize;
    unsafe {
        asm!("csrci sstatus, 1 << 1" : "=r"(sstatus) ::: "volatile");
    }
    sstatus & (1 << 1)
}

#[inline(always)]
pub fn restore(flags: usize) {
    // 根据 flag 设置中断
    unsafe {
        asm!("csrs sstatus, $0" :: "r"(flags) :: "volatile");
    }
}
