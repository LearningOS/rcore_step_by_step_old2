use riscv::register::{mcause::Mcause, sstatus::Sstatus};

#[repr(C)]
#[derive(Debug)]
pub struct TrapFrame {
    pub x: [usize; 32],   // General registers
    pub sstatus: Sstatus, // Supervisor Status Register
    pub sepc: usize,      // Supervisor exception program counter
    pub stval: usize,     // Supervisor trap value
    pub scause: Mcause,   // Scause register: record the cause of exception/interrupt/trap
}

impl TrapFrame {
    pub fn increase_sepc(&mut self) {
        self.sepc += 4;
    }
}
