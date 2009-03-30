;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; start.asm
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
; This is the kernel's entry point. We could either call main here,
; or we can use this to setup the stack or other nice stuff, like
; perhaps setting up the GDT and segments. Please note that interrupts
; are disabled at this point.
[BITS 32]
global start
start:
    mov esp, _sys_stack     ; This points the stack to our new stack area
    call init_real_mode
    jmp stublet

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

; This section 
init_real_mode:

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
