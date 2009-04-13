;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; start.asm
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
; This is the kernel's entry point. We could either call main here,
; or we can use this to setup the stack or other nice stuff, like
; perhaps setting up the GDT and segments. Please note that interrupts
; are disabled at this point.
[BITS 32]
global start
global set_gdt
global set_idt

; This part MUST be 4byte aligned, so we solve that issue using 'ALIGN 4'
ALIGN 4
mboot:
  ; Multiboot macros to make a few lines later more readable
  MULTIBOOT_PAGE_ALIGN    equ 1<<0
  MULTIBOOT_MEMORY_INFO       equ 1<<1
  MULTIBOOT_HEADER_MAGIC  equ 0x1BADB002
  MULTIBOOT_HEADER_FLAGS      equ MULTIBOOT_PAGE_ALIGN | MULTIBOOT_MEMORY_INFO
  MULTIBOOT_CHECKSUM      equ -(MULTIBOOT_HEADER_MAGIC + MULTIBOOT_HEADER_FLAGS)

  ; This is the GRUB Multiboot header. A boot signature
  dd MULTIBOOT_HEADER_MAGIC
  dd MULTIBOOT_HEADER_FLAGS
  dd MULTIBOOT_CHECKSUM

; this descriptor stores the location and size of the Global Descriptor Table,
; and is used in the following function
gdtr:
  dw 0 ; the limit, or size of thr Global Descriptor Table
  dd 0 ; the offset, or location of the Global Descriptor Tablea

idtr:
  dw 0 ; the size of the Interrupt Descriptor Table
  dd 0 ; the location of the Interrupt Descriptor Table

SECTION .text
start:
    mov esp, _sys_stack     ; This points the stack to our new stack area
    jmp stublet
set_gdt:  
  mov eax, [esp + 4] ; copy the address of the gdt into eax.
  mov [gdtr + 2], eax ; copy the address to the location part of the gdtr struct.
  mov ax, [esp + 8] ; copy the size of the global descriptor table into ax
  mov [gdtr], ax ; copy the size to the size part of the gdtr struct
  lgdt [gdtr] ; load the global descriptor table
ret

set_idt:  
  mov eax, [esp + 4] ; copy the address of the idt into eax.
  mov [idtr + 2], eax ; copy the address to the location part of the idtr struct.
  mov ax, [esp + 8] ; copy the size of the idt into ax
  mov [idtr], ax ; copy the size to the size part of the idtr struct
  lidt [idtr] ; load the global descriptor table
ret

; A call to main (the C kernel) followed by an infinite loop (jmp $)
stublet:
  push ebx                ; push the multiboot structure
  push eax                ; push the mulitboot magic value
  EXTERN cmain            ; start of our kernel
  call cmain
  jmp $

; Here is the definition of our BSS section. Right now, we'll use
; it just to store the stack. Remember that a stack actually grows
; downwards, so we declare the size of the data before declaring
; the identifier '_sys_stack'
SECTION .bss
resb 16384               ; This reserves 16KBytes of memory here
_sys_stack:
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;                                                                            
