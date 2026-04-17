OUTPUT_ARCH(riscv)
ENTRY(_start)

BASE_ADDRESS = 0x80000000;

SECTIONS
{
    . = BASE_ADDRESS;
    
    .text : {
        *(.text._start) /* Đảm bảo _start nằm đầu tiên */
        *(.text .text.*)
    }

    .rodata : {
        *(.rodata .rodata.*)
    }

    .data : {
        *(.data .data.*)
    }

    .bss : {
        *(.bss .bss.*)
        . = ALIGN(16);
        _stack_start = .;
        . += 0x10000; /* 64KB stack */
        _stack_end = .;
    }

    . = ALIGN(4096);
    _end = .;

    /DISCARD/ : {
        *(.eh_frame)
    }
}