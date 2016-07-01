ENTRY(start)

SECTIONS {
    . = 1M;

    .boot :
    {
        /* ensure multiboot header is at the beginning. */
        *(.multiboot_header)
    }

    .text :
    {
        *(.text)
    }
}
