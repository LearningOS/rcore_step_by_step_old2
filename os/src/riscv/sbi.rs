#[naked]
#[inline(never)]
fn ecall(_a0: u32, _a1: u32, _a2: u32, _a3: u32, _a4: u32, _a5: u32, _a6: u32, _a7: u32) {
    unsafe { asm!("ecall" :: : "a0" : "volatile") }
}

pub fn set_timer(stime_value: u64) {
    ecall((stime_value&0xffffffff) as u32, (stime_value>>32) as u32, 0, 0, 0, 0, 0, 0);
}

pub fn clear_ipi() {
    ecall(0, 0, 0, 0, 0, 0, 0, 3);
}

pub fn send_ipi(hart_mask_pointer: u32) {
    ecall(hart_mask_pointer, 0, 0, 0, 0, 0, 0, 4);
}

pub fn shutdown() {
    ecall(0, 0, 0, 0, 0, 0, 0, 8);
}

pub fn send_ipi_to_hart(hart: u32) {
    let mask: u32 = 1 << hart;
    send_ipi(&mask as *const u32 as u32);
}
