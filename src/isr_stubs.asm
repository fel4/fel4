; Assembly stubs for Interrupt Service Routines
[BITS 32]
global exception_stub
global exception0_stub
global exception1_stub
global exception3_stub
global exception4_stub
global exception5_stub
global exception6_stub
global exception7_stub
global exception8_stub
global exception9_stub
global exception10_stub
global exception11_stub
global exception12_stub
global exception13_stub
global exception14_stub
global exception16_stub
global interrupt_stub

SECTION .text

exception_stub:
  pushad
  EXTERN exception_handler
  call exception_handler
  popad
  iretd

exception0_stub:
  pushad
  EXTERN exception0_handler
  call exception0_handler
  popad
  iretd

exception1_stub:
  pushad
  EXTERN exception1_handler
  call exception1_handler
  popad
  iretd

exception3_stub:
  pushad
  EXTERN exception3_handler
  call exception3_handler
  popad
  iretd

exception4_stub:
  pushad
  EXTERN exception4_handler
  call exception4_handler
  popad
  iretd

exception5_stub:
  pushad
  EXTERN exception5_handler
  call exception5_handler
  popad
  iretd

exception6_stub:
  pushad
  EXTERN exception6_handler
  call exception6_handler
  popad
  iretd

exception7_stub:
  pushad
  EXTERN exception7_handler
  call exception7_handler
  popad
  iretd

exception8_stub:
  pop eax
  pushad
  push eax
  EXTERN exception8_handler
  call exception8_handler
  popad
  iretd

exception9_stub:
  pushad
  EXTERN exception9_handler
  call exception9_handler
  popad
  iretd

exception10_stub:
  pushad
  EXTERN exception10_handler
  call exception10_handler
  popad
  iretd

exception11_stub:
  pushad
  EXTERN exception11_handler
  call exception11_handler
  popad
  iretd

exception12_stub:
  pushad
  EXTERN exception12_handler
  call exception12_handler
  popad
  iretd

exception13_stub:
  pushad
  EXTERN exception13_handler
  call exception13_handler
  popad
  iretd

exception14_stub:
  pushad
  EXTERN exception14_handler
  call exception14_handler
  popad
  iretd

exception16_stub:
  pushad
  EXTERN exception16_handler
  call exception16_handler
  popad
  iretd


interrupt_stub:
  pushad
  EXTERN interrupt_handler
  call interrupt_handler
  popad
  iretd
