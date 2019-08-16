use bbl::sbi::set_timer;
use riscv::register::{sie, time, timeh};

pub static mut TICK: usize = 0;

pub fn init() {
    unsafe {
        TICK = 0;
        sie::set_stimer();
    }
    clock_set_next_event();
    println!("++++setup timer !++++");
}

const TIMEBASE: u64 = 100000;

pub fn clock_set_next_event() {
    set_timer(get_cycle() + TIMEBASE);
}

fn get_cycle() -> u64 {
    loop {
        let hi = timeh::read();
        let lo = time::read();
        let tmp = timeh::read();
        if hi == tmp {
            return ((hi as u64) << 32) | (lo as u64);
        }
    }
}
