    .section .text.entry
    .globl _start
_start:
    # 1. enable paging
    # satp = (1 << 31) | PPN(boot_page_table_sv32)
    lui     t0, %hi(boot_page_table_sv32)
    srli    t0, t0, 12
    li      t1, 1 << 31
    or      t0, t0, t1
    csrw    satp, t0
    sfence.vma

    # 2. setup stack pointer
    lui sp, %hi(bootstacktop)

    # 3. call rust_main
    call    rust_main

    .section .bss.stack
    .align 12  #PGSHIFT
    .global bootstack
bootstack:
    .space 4096 * 16
    .global bootstacktop
bootstacktop:

    .section .data
    .align 12
boot_page_table_sv32:
    .zero 4 * 512
    # 0x80000000 -> 0x80000000 (4M * 2)
    .word (0x80000 << 10) | 0xcf # VRWXAD
    .word (0x80400 << 10) | 0xcf # VRWXAD
    .zero 4 * 510
