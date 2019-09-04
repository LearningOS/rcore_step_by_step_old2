#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct TrapFrame {
    x31: usize, // t6
    x30: usize, // t5
    x29: usize, // t4
    x28: usize, // t3
    x27: usize, // s11
    x26: usize, // s10
    x25: usize, // s9
    x24: usize, // s8
    x23: usize, // s7
    x22: usize, // s6
    x21: usize, // s5
    x20: usize, // s4
    x19: usize, // s3
    x18: usize, // s2
    x17: usize, // a7
    x16: usize, // a6
    x15: usize, // a5
    x14: usize, // a4
    x13: usize, // a3
    x12: usize, // a2
    x11: usize, // a1
    x10: usize, // a0
    x9: usize,  // s1
    x8: usize,  // s0/fp
    x7: usize,  // t2
    x6: usize,  // t1
    x5: usize,  // t0
    x4: usize,  // tp
    x3: usize,  // gp
    x2: usize,  // sp
    x1: usize,  // ra
}

/// Trap handler
/// Return the new epc
#[no_mangle]
pub extern "C" fn trap_handler(
    trap_frame: &mut TrapFrame,
    cause: usize,
    mut pc: usize,
    tval: usize,
) -> usize {
    match cause {
        11 => {
            // sbi
            let call = trap_frame.x17; // a7
            let arg1 = trap_frame.x10; // a0
            let arg2 = trap_frame.x11; // a1
            let arg3 = trap_frame.x12; // a2

            // return code at a0
            trap_frame.x10 = match call {
                0 => sbi_set_timer(arg1),
                1 => sbi_console_putchar(arg1),
                2 => sbi_console_getchar(),
                3 => sbi_clear_ipi(),
                4 => sbi_send_ipi(arg1),
                5 => sbi_remote_fence_i(arg1),
                6 => sbi_remote_sfence_vma(arg1, arg2, arg3),
                _ => (-38isize as usize), // NOSYS
            };

            // skip ecall instruction
            pc += 4;
        }
        _ => unimplemented!(
            "cause {} with epc {:#X} tval {:#X} trapframe {:x?}",
            cause,
            pc,
            tval,
            trap_frame
        ),
    }

    pc
}

fn sbi_set_timer(time: usize) -> usize {
    // do nothing
    println!("sbi: set timer {}", time);
    0
}

fn sbi_console_putchar(ch: usize) -> usize {
    // do nothing
    //println!("sbi: put char {}", ch);
    print!("{}", ch as u8 as char);
    0
}

fn sbi_console_getchar() -> usize {
    // do nothing
    println!("sbi: get char");
    0
}

fn sbi_clear_ipi() -> usize {
    // do nothing
    println!("sbi: clear ipi");
    0
}

fn sbi_send_ipi(hart_mask: usize) -> usize {
    // do nothing
    println!("sbi: send ipi hart mask {}", hart_mask);
    0
}

fn sbi_remote_fence_i(hart_mask: usize) -> usize {
    // do nothing
    println!("sbi: remote fence i hart mask {}", hart_mask);
    0
}

fn sbi_remote_sfence_vma(hart_mask: usize, start: usize, size: usize) -> usize {
    // do nothing
    println!(
        "sbi: remote sfence vma hart mask {} start {:#X} size {:#X}",
        hart_mask, start, size
    );
    0
}
