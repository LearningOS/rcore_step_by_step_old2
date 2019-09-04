.section .text.entry
.global boot
boot:
    li x1, 0
    li x2, 0
    li x3, 0
    li x4, 0
    li x5, 0
    li x6, 0
    li x7, 0
    li x8, 0
    li x9, 0
    # skip a0 and a1
    # a0 is hart id
    # a1 is dtb address
    li x12, 0
    li x13, 0
    li x14, 0
    li x15, 0
    li x16, 0
    li x17, 0
    li x18, 0
    li x19, 0
    li x20, 0
    li x21, 0
    li x22, 0
    li x23, 0
    li x24, 0
    li x25, 0
    li x26, 0
    li x27, 0
    li x28, 0
    li x29, 0
    li x30, 0
    li x31, 0
    csrw mscratch, x0

    lui t0, %hi(trap_vector)
    addi t0, t0, %lo(trap_vector)
    csrw mtvec, t0

    # ensure mtvec is set correctly
try:
    csrr t1, mtvec
    bne t0, t1, try

    # setup boot stack space
    add t0, a0, 1
    slli t0, t0, 16
    lui sp, %hi(bootstack)
    add sp, sp, t0

    # first hart first
    bnez a0, other_hart
    j boot_first_hart

other_hart:
    wfi

.section .bss.stack
.align 12 # PAGE_SIZE
.global bootstack
bootstack:
    # 8 * 64KB stack
    .space 1024 * 64 * 8
.global bootstacktop
bootstacktop: