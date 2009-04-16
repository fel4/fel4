;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; klib.asm
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
; This file contains assembly functions to allow the rest of the kernel to be
; written in C.

[BITS 32]
global enable_interrupts
global enable_nmi
global disable_interrupts
global disable_nmi
global set_gdt
global set_idt
global reload_segments

; this descriptor stores the location and size of the Global Descriptor Table,
; and is used in the following function
gdtr:
  dw 0 ; the limit, or size of thr Global Descriptor Table
  dd 0 ; the offset, or location of the Global Descriptor Table
  
; this descriptor stores the location and size of the Interrupt Descriptor Table,
; and is used in the following function
idtr:
  dw 0 ; the size of the Interrupt Descriptor Table
  dd 0 ; the location of the Interrupt Descriptor Table

SECTION .text

; This function allows C code to enable interrupts
enable_interrupts:
  sti
ret

; This function allows C code to disable interrupts
disable_interrupts:
  cli
ret

; disable non-maskable interrupts
disable_nmi:
  in ax, 0x70
  or ax, 0x80
  out 0x70, ax
ret

; enable non-maskable interrupts
enable_nmi:
  in ax, 0x70
  and ax, 0x7f
  out 0x07, ax
ret


reload_segments:
  ; Reload CS register containing code selector:
  jmp   0x08:reload_CS ; 0x08 points at the new code selector
reload_CS:
  ; Reload data segment registers:
  mov   ax, 0x10 ; 0x10 points at the new data selector
  mov   ds, ax
  mov   es, ax
  mov   fs, ax
  mov   gs, ax
  mov   ss, ax
ret

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

