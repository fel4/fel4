ENTRY(start)

SECTIONS {
    . = 1M;

    .rodata :
    {
        /* ensure multiboot header is at the beginning. */
        KEEP(*(.multiboot_header))
        *(.rodata .rodata.*)
    }

    .text :
    {
        *(.text .text.*)
        . = ALIGN(4K);
    }

    .data :
    {
        *(.data .data.*)
        . = ALIGN(4K);
    }

    .bss :
    {
        *(.bss *.bss.*)
        . = ALIGN(4K);
    }

    .data.rel.ro : ALIGN(4K)
    {
        *(.data.rel.ro.local*) *(.data.rel.ro .data.rel.ro.*)

    }


}
