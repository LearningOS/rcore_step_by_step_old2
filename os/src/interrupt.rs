use crate::context::TrapFrame;

global_asm!(include_str!("trap/trap.asm"));

use riscv::register::{stvec, sstatus};
#[no_mangle]
pub fn init() {
    extern {
        fn __alltraps();
    }
    unsafe {
        sstatus::set_sie();
        stvec::write(__alltraps as usize, stvec::TrapMode::Direct);
    }
}

impl TrapFrame {
    pub fn increase_sepc(self: &mut Self) {
        self.sepc = self.sepc + 4;
    }
}

use riscv::register::scause::Trap;
use riscv::register::scause::Exception;
use riscv::register::scause::Interrupt;
use crate::clock::{ TICK, clock_set_next_event };

#[no_mangle]
pub extern "C" fn rust_trap(tf: &mut TrapFrame) {
    match tf.scause.cause() {
        Trap::Exception(Exception::Breakpoint) => breakpoint(),
        Trap::Interrupt(Interrupt::SupervisorTimer) => super_timer(),
        _ => panic!("unexpected trap"),
    }
}

fn breakpoint() {
    panic!("a breakpoint set by kernel");
}

fn super_timer() {
    // 响应当前时钟中断的同时，手动设置下一个时钟中断。这一函数调用同时清除了 MTIP ，使得 CPU 知道当前这个中断被正确处理。
    clock_set_next_event();
    unsafe{
        TICK = TICK + 1;
        if(TICK % 100 == 0) {
            println!("100 ticks!");
        }
    }
}