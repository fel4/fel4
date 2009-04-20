#include <macros.h>
#include <system.h>


idt_entry_t idt[NUM_INTERRUPTS]; /* declared here, initialzed in init_idt() */

extern void exception_stub();
extern void interrupt_stub();

/* 
 * this function initializes the interrupt descriptor table to point every
 * interrupt to execute def_handler.
 */
void init_idt() {
  
  int i = 0;

  for ( i = 0; i < NUM_INTERRUPTS; i++ ) {
    if ( i < 16 ) {
      if ( i == 2 || i == 15 ) {
        set_handler(idt, i , 0, NULL);
        continue;
      }
      set_handler(idt, i, 1, exception_stub);
      continue;
    }
    set_handler(idt, i, 1, interrupt_stub);
  }
}

void set_handler(idt_entry_t table[], int inter_num, int present, void (*handler)(void)) {

  unsigned short low_addr = ((unsigned long) handler) & 0xFFFF;
  unsigned short high_addr = (((unsigned long) handler) & 0xFFFF0000) >> 16;

  unsigned char type = 0;

  type  = present << 7;
  type |= 0xE;

  table[inter_num].offset_1  = low_addr;
  table[inter_num].offset_2  = high_addr;
  table[inter_num].zero      = 0;
  table[inter_num].type_attr = type;
  table[inter_num].selector  = 0x8;

}
