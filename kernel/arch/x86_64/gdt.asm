section .text
bits 32

section .rodata
global gdt64, gdt64.code, gdt64.data, gdt64.user_code, gdt64.user_data, gdt64.pointer
gdt64:
    dq 0 ; zero entry
.code: equ $ - gdt64 ; new
    dq (1<<44) | (1<<47) | (1<<41) | (1<<43) | (1<<53) ; code segment
.data: equ $ - gdt64 ; new
    dq (1<<44) | (1<<47) | (1<<41) ; data segment
.user_code: equ $ - gdt64 ; new
    dq (1<<44) | (3<<45) | (1<<47) | (1<<41) | (1<<43) | (1<<53) ; userspace code segment
.user_data: equ $ - gdt64 ; new
    dq (1<<44) | (3<<45) | (1<<47) | (1<<41) ; userspace data segment
.pointer:
    dw $ - gdt64 - 1
    dq gdt64
