#include <system.h>

void exception_handler(void) {
  kprintf("CPU Exception!\n");
}

void interrupt_handler(void) {
  kprintf("saw an interrupt\n");
}
