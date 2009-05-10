; Assembly stubs for Interrupt Service Routines
[BITS 32]
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
global timer_stub
global keyboard_stub
global ipc_stub
global id_nearest_stub
global fpage_unmap_stub
global thread_switch_stub
global thread_schedule_stub
global lthread_ex_regs_stub
global task_new_stub


SECTION .text

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

timer_stub:
  pushad
  EXTERN timer_handler
  call timer_handler
  popad
  iretd

keyboard_stub:
  pushad
  EXTERN keyboard_handler
  call keyboard_handler
  popad
  iretd

ipc_stub:
  EXTERN ipc_handler
  call ipc_handler
  iretd

id_nearest_stub:
  EXTERN id_nearest_handler
  call id_nearest_handler
  iretd

fpage_unmap_stub:
  EXTERN fpage_unmap_handler
  call fpage_unmap_handler
  iretd

thread_switch_stub:
  EXTERN thread_switch_handler
  call thread_switch_handler
  iretd

thread_schedule_stub:
  EXTERN thread_schedule_handler
  call thread_schedule_handler
  iretd

lthread_ex_regs_stub:
  EXTERN lthread_ex_regs_handler
  call lthread_ex_regs_handler
  iretd

task_new_stub:
  EXTERN task_new_handler
  call task_new_handler
  iretd
