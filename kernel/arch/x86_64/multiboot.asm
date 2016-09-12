extern error

global check_multiboot

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


section .text
bits 32

check_multiboot:
    cmp eax, 0x36d76289
    jne .no_multiboot
    ret

.no_multiboot:
    mov al, "0"
    jmp error
