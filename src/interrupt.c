#include <macros.h>
#include <system.h>

void exception_handler(void) {
  kprintf("CPU Exception!\n");
}

void exception0_handler(void) {
  kprintf("0 -- Divide Error!\n");
}
  
void exception1_handler(void) {
  kprintf("1 -- Debug Exception!\n");
}

void exception3_handler(void) {
  kprintf("3 -- Breakpoint!\n");
}

void exception4_handler(void) {
  kprintf("4 -- Overflow!\n");
}

void exception5_handler(void) {
  kprintf("5 -- Bounds Check Error!\n");
}

void exception6_handler(void) {
  kprintf("6 -- Invalid Opcode!\n");
}

void exception7_handler(void) {
  kprintf("7 -- Coprocessor Not Available\n");
}

void exception8_handler(void) {
  kprintf("8 -- Double Fault!\n");
}

void exception9_handler(void) {
  kprintf("9 -- Coprocessor Segment Overrun!\n");
}

void exception10_handler(void) {
  kprintf("10 -- Invalid TSS!\n");
}

void exception11_handler(void) {
  kprintf("11 -- Segment Not Present!\n");
}

void exception12_handler(void) {
  kprintf("12 -- Stack Exception!\n");
}

void exception13_handler(void) {
  kprintf("13 -- General Protection Exception!\n");
}

void exception14_handler(void) {
  kprintf("14 -- Page Fault!\n");
}

void exception16_handler(void) {
  kprintf("16 -- Coprocessor Error!\n");
}

void interrupt_handler(void) {
  kprintf("saw an interrupt\n");
}
