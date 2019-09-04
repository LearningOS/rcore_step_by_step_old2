OUTPUT_ARCH(riscv)
ENTRY(boot)

SECTIONS {
    . = 0x80000000;
    .text : {
        *(.text.entry)
        *(.text .text.*)
    }

    .rodata : {
        *(.rodata .rodata.*)
    }

    .data : {
        *(.data .data.*)
    }

    .stack : {
        *(.bss.stack)
    }

    . = ALIGN(4);
    .bss : {
        PROVIDE(sbss = .);
        *(.bss .bss.*)
        PROVIDE(ebss = .);
    }
}
