;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; klib.asm
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
; This file contains assembly functions too allow the rest of the kernel to be
; written in C.

global enable_interrupts
global enable_nmi
global disable_interrupts
global disable_nmi


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
