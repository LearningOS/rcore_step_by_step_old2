use riscv::register::{scause, sepc, stvec};

pub fn init() {
    unsafe {
        stvec::write(trap_handler as usize, stvec::TrapMode::Direct);
    }
    println!("++++setup interrupt !++++");
}

fn trap_handler() -> ! {
    let cause = scause::read().cause();
    let epc = sepc::read();
    println!("trap: cause: {:?}, epc: {:#x}", cause, epc);
    panic!("trap");
}
