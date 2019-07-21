    .section .text.entry
    .globl _start
_start:
    lui sp, %hi(bootstacktop)

    call rust_main

    .section .bss.stack
    .align 12  #PGSHIFT
    .global bootstack
bootstack:
    .space 4096 * 16
    .global bootstacktop
bootstacktop: