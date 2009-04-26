#include <system.h>

void handle_kbd_event(void) {

  unsigned char scancode;

  scancode = inportb(0x60);

  kprintf("saw scancode %x\n", scancode);
}
