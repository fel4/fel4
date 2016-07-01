section .multiboot_header
header_start:
    dd 0xe85250d6                   ; multiboot2 magic number
    dd 0                            ; arch 0 (protected mode x86)
    dd header_end - header_start    ; header length
    dd 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start))

    ; optional multiboot tags here


    ; required end tag
    dw 0    ; type
    dw 0    ; flags
    dw 8    ; size
header_end:
