use crate::context::TrapFrame;

global_asm!(include_str!("trap/trap.asm"));

use riscv::register::{stvec, sscratch, sie, sstatus};
#[no_mangle]
pub fn init() {
    extern {
        fn __alltraps();
    }
    unsafe {
        sscratch::write(0); // 给中断 asm 初始化
        sstatus::set_sie();
        stvec::write(__alltraps as usize, stvec::TrapMode::Direct);
        sie::set_sext(); // 开外部中断（串口）
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
        Trap::Exception(Exception::UserEnvCall) => syscall(tf),
        Trap::Interrupt(Interrupt::SupervisorExternal) => {
            let ch = bbl::sbi::console_getchar() as u8 as char;
            external(ch as u8);
        },
        _ => panic!("unexpected trap"),
    }
}

pub const SYS_WRITE: usize = 64;
pub const SYS_EXIT: usize = 93;

fn syscall(tf: &mut TrapFrame) {
    let ret = crate::syscall::syscall(
        tf.x[17],
        [tf.x[10], tf.x[11], tf.x[12]],
        tf,
    );
    tf.sepc += 4;
    tf.x[10] = ret as usize;
}

fn breakpoint() {
    panic!("a breakpoint set by kernel");
}

use crate::process::tick;
fn super_timer() {
    clock_set_next_event();
    unsafe{
        TICK = TICK + 1;
        // if TICK % 100 == 0 {
        //     println!("100 ticks!");
        // }
    }
    tick();
}

#[inline(always)]
pub fn enable_and_wfi() {    // 使能中断并等待中断
    unsafe {
        asm!("csrsi sstatus, 1 << 1; wfi" :::: "volatile");
    }
}

#[inline(always)]
pub fn disable_and_store() -> usize {    // 禁用中断并返回当前中断状态
    let sstatus: usize;
    unsafe {
        asm!("csrci sstatus, 1 << 1" : "=r"(sstatus) ::: "volatile");
    }
    sstatus & (1 << 1)
}

#[inline(always)]
pub fn restore(flags: usize) {    // 根据 flag 设置中断
    unsafe {
        asm!("csrs sstatus, $0" :: "r"(flags) :: "volatile");
    }
}

fn external(ch: u8) {
    crate::fs::stdio::STDIN.push(ch as char);
}
