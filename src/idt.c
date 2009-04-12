#include <idt.h>

idt_entry_t idt[NUM_INTERRUPTS]; /* declared here, initialzed in init_idt() */

/* 
 * this function initializes the interrupt descriptor table to point every
 * interrupt to execute def_handler.
 */
void init_idt(idt_entry_t table[], void (*def_handler)(void), int count) {
  
  int i = 0;

  for ( i = 0; i < count; i++ ) {
    set_handler(table, i, def_handler);
  }
}

void set_handler(idt_entry_t table[], int inter_num, void (*handler)(void)) {

  unsigned short low_addr = ((unsigned long) handler) & 0xFFFF;
  unsigned short high_addr = (((unsigned long) handler) & 0xFFFF0000) >> 16;

  unsigned char type = 0;

  type  = 1 << 7;
  type |= 0xE;

  table[inter_num].offset_1  = low_addr;
  table[inter_num].offset_2  = high_addr;
  table[inter_num].zero      = 0;
  table[inter_num].type_attr = type;
  table[inter_num].selector  = 1;

}
