; Assembly stubs for Interrupt Service Routines
[BITS 32]
global exception_stub
global interrupt_stub

SECTION .text

exception_stub:
  pushad
  EXTERN exception_handler
  call exception_handler
  popad
  iretd

interrupt_stub:
  pushad
  EXTERN interrupt_handler
  call interrupt_handler
  popad
  iretd
