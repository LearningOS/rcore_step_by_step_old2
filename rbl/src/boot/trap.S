.section .text
.global trap_vector
.align 4
trap_vector:
    # if sp = 0, the trap comes from machine mode
    csrrw sp, mscratch, sp
    beqz sp, trap_from_machine_mode
    
    # save the context
    sw x1, -1*4(sp)
    # x2 = sp, get it from mscratch
    csrrw x1, mscratch, x0
    sw x1, -2*4(sp)
    sw x3, -3*4(sp)
    sw x4, -4*4(sp)
    sw x5, -5*4(sp)
    sw x6, -6*4(sp)
    sw x7, -7*4(sp)
    sw x8, -8*4(sp)
    sw x9, -9*4(sp)
    sw x10, -10*4(sp)
    sw x11, -11*4(sp)
    sw x12, -12*4(sp)
    sw x13, -13*4(sp)
    sw x14, -14*4(sp)
    sw x15, -15*4(sp)
    sw x16, -16*4(sp)
    sw x17, -17*4(sp)
    sw x18, -18*4(sp)
    sw x19, -19*4(sp)
    sw x20, -20*4(sp)
    sw x21, -21*4(sp)
    sw x22, -22*4(sp)
    sw x23, -23*4(sp)
    sw x24, -24*4(sp)
    sw x25, -25*4(sp)
    sw x26, -26*4(sp)
    sw x27, -27*4(sp)
    sw x28, -28*4(sp)
    sw x29, -29*4(sp)
    sw x30, -30*4(sp)
    sw x31, -31*4(sp)

    # args
    addi sp, sp, -31*4
    mv a0, sp
    csrr a1, mcause
    csrr a2, mepc
    csrr a3, mtval

    jal trap_handler

    # update mepc
    csrw mepc, a0

    # save sp to mscratch, so we know kernel sp on next trap
    addi sp, sp, 31*4
    csrw mscratch, sp

    lw x1, -1*4(sp)
    # x2 is sp, restore it later
    lw x3, -3*4(sp)
    lw x4, -4*4(sp)
    lw x5, -5*4(sp)
    lw x6, -6*4(sp)
    lw x7, -7*4(sp)
    lw x8, -8*4(sp)
    lw x9, -9*4(sp)
    lw x10, -10*4(sp)
    lw x11, -11*4(sp)
    lw x12, -12*4(sp)
    lw x13, -13*4(sp)
    lw x14, -14*4(sp)
    lw x15, -15*4(sp)
    lw x16, -16*4(sp)
    lw x17, -17*4(sp)
    lw x18, -18*4(sp)
    lw x19, -19*4(sp)
    lw x20, -20*4(sp)
    lw x21, -21*4(sp)
    lw x22, -22*4(sp)
    lw x23, -23*4(sp)
    lw x24, -24*4(sp)
    lw x25, -25*4(sp)
    lw x26, -26*4(sp)
    lw x27, -27*4(sp)
    lw x28, -28*4(sp)
    lw x29, -29*4(sp)
    lw x30, -30*4(sp)
    lw x31, -31*4(sp)

    lw x2, -2*4(sp)
    mret

trap_from_machine_mode:
    wfi