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

void exception8_handler(unsigned long errorcode) {
  unsigned short err = 0;
  kprintf("8 -- Double Fault!\n");
  kprintf("errorcode = %d\n", errorcode);
  err = (unsigned short) errorcode & 0x0000FFFF;
  kprintf("ext bit: %d\n", errorcode & 1);
  kprintf("i bit: %d\n", (errorcode >> 1) & 1);
  if ( ! (errorcode & ( 1 << 1) ) ) {
    kprintf("ti bit: %d\n", (errorcode >> 2 )& 1);
  }
  kprintf("source selector: 0x%x\n", err >> 3 );
  
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
  outportb(0x20, 0x20);
}

void timer_handler(void) {
  /* TODO -- add call to process scheduler. */
  outportb(0x20, 0x20);
}

void keyboard_handler(void) { 
  handle_kbd_event();
  outportb(0x20,0x20);
}
