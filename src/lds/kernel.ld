OUTPUT_ARCH(riscv)
ENTRY(_start)

/* QEMU virt machine: RAM bắt đầu từ 0x80000000
   OpenSBI chiếm 0x80000000–0x80200000
   Kernel load vào 0x80200000                    */

BASE_ADDRESS = 0x80200000;

SECTIONS {
    . = BASE_ADDRESS;

    .text : {
        /* Entry point phải là đầu tiên */
        KEEP(*(.text.boot))
        *(.text .text.*)
    }

    . = ALIGN(4096);
    .rodata : {
        *(.rodata .rodata.*)
    }

    . = ALIGN(4096);
    .data : {
        *(.data .data.*)
    }

    . = ALIGN(4096);
    .bss : {
        _bss_start = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
        _bss_end = .;
    }

    . = ALIGN(4096);
    /* Stack - 64KB */
    _stack_bottom = .;
    . += 0x10000;
    _stack_top = .;

    /* Kernel end */
    _kernel_end = .;
}