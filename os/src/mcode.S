// This is the default M-mode trap handler. It forwards timer interrupts to
// S-mode and loops for all other interrupt and exception causes.
.align 4
mtrap_entry:
	csrrw sp, mscratch, sp
	addi sp, sp, -64
	sw ra, 0(sp)
	sw t0, 4(sp)
	sw t1, 8(sp)
	sw t2, 12(sp)
	sw t3, 16(sp)
	sw t4, 20(sp)
	sw t5, 24(sp)
	sw t6, 28(sp)
	sw a0, 32(sp)
	sw a1, 36(sp)
	sw a2, 40(sp)
	sw a3, 44(sp)
	sw a4, 48(sp)
	sw a5, 52(sp)
	sw a6, 56(sp)
	sw a7, 60(sp)

	csrr t0, mcause
	bgez t0, exception

	li t1, 0x80000003
	beq t0, t1, msoftware_interrupt

	li t1, 0x80000007
	beq t0, t1, mtimer_interrupt

	li t1, 0x8000000b
	beq t0, t1, mexternal_interrupt

unknown_cause:
	j unknown_cause

msoftware_interrupt:
	csrsi mip, 0x2 // mip.ssip = 1

	csrr t0, mhartid
	slli t0, t0, 2
	li t1, 0x2000000 //CLINT_BASE  0x2000000
	add t1, t0, t1
	sw zero, 0(t1)  // msoft[hartid] = zero

	j return

mtimer_interrupt:
	li t0, 0x20
	csrs mip, t0 // mip.stip = 1

	csrr t0, mhartid
	slli t0, t0, 3    // t0 = hartid * 8 bytes (hartid * one 64-bit word)
	li t1, 0x2004000  // mtimecmp  CLINT_BASE + 0x4000
	add t1, t0, t1    // # t1 = mtimecmp + hartid * 8 = address of this CPU's mtimecmp
    // for RV64
	// li t0, 0xffffffffffff
	//sd t0, 0(t1)  // mtimecmp[hartid] = 2^48 - 1

    // for RV32
	li t0, -1            // for RV32, manuals recommend setting all high bits first
	sw t0, 4(t1)
	sw t0, 0(t1)         // then write low 32-bit word
    li t0, 0xffff
	sw t0, 4(t1)         // then the high 32-bit word,  mtimecmp[hartid] = 2^48 - 1
	j return

mexternal_interrupt:
	j mexternal_interrupt

exception:
	li t1, 9
	beq t0, t1, ecall_exception
	call forward_exception
	j return

ecall_exception:
	csrr t0, mepc
	addi t0, t0, 4
	csrw mepc, t0

	beqz a7, sbi_set_timer

	li t1, 3
	beq a7, t1, sbi_clear_ipi

	li t1, 4
	beq a7, t1, sbi_send_ipi

	li t1, 8
	beq a7, t1, sbi_shutdown

	j unknown_cause

sbi_set_timer:
	csrr t0, mhartid
	slli t0, t0, 3
	li t1, 0x2004000
	add t1, t0, t1
	sw a0, 0(t1)
	sw a1, 4(t1)  // mtimecmp[hartid] = a1, a0

	li t0, 0x20
	csrc mip, t0 // mip.stip = 0

	li a0, 0
	j return_with_value

sbi_clear_ipi:
	csrci mip, 0x2
	li a0, 0
	j return_with_value

sbi_send_ipi:
	li t2, 1 << 17 // t2 = MPRV
	csrrs t1, mstatus, t2
	//ld t0, 0(a0)  //RV64 ???
    lw t0, 0(a0)
	csrw mstatus, t1

	li t2, (0x02000000 - 4)
1:	addi t2, t2, 4
	andi t1, t0, 0x1
	srli t0, t0, 1
	beqz t1, 1b
	sw t1, 0(t2)
	bnez t0, 1b

	li a0, 0
	j return_with_value

sbi_shutdown:
	j sbi_shutdown

return:
	lw a0, 32(sp)
return_with_value:
	lw ra, 0(sp)
	lw t0, 4(sp)
	lw t1, 8(sp)
	lw t2, 12(sp)
	lw t3, 16(sp)
	lw t4, 20(sp)
	lw t5, 24(sp)
	lw t6, 28(sp)
	lw a1, 36(sp)
	lw a2, 40(sp)
	lw a3, 44(sp)
	lw a4, 48(sp)
	lw a5, 52(sp)
	lw a6, 56(sp)
	lw a7, 60(sp)
	addi sp, sp, 64
	csrrw sp, mscratch, sp
	mret
