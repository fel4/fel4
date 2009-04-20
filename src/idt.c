#include <macros.h>
#include <system.h>


idt_entry_t idt[NUM_INTERRUPTS]; /* declared here, initialzed in init_idt() */

extern void exception0_stub();
extern void exception1_stub();
extern void exception3_stub();
extern void exception4_stub();
extern void exception5_stub();
extern void exception6_stub();
extern void exception7_stub();
extern void exception8_stub();
extern void exception9_stub();
extern void exception10_stub();
extern void exception11_stub();
extern void exception12_stub();
extern void exception13_stub();
extern void exception14_stub();
extern void exception16_stub();
extern void interrupt_stub();

/* 
 * this function initializes the interrupt descriptor table to point every
 * interrupt to execute def_handler.
 */
void init_idt() {
  
  int i = 0;

  for ( i = 0; i < NUM_INTERRUPTS; i++ ) {
    switch ( i ) {
      case 0:
        set_handler(idt, i, 1, exception0_stub);
        break;
      case 1:
        set_handler(idt, i, 1, exception1_stub);
        break;
      case 2: /* interrupt 2 is reserved */
        set_handler(idt, i, 0, NULL);
        break;
      case 3:
        set_handler(idt, i, 1, exception3_stub);
        break;
      case 4:
        set_handler(idt, i, 1, exception4_stub);
        break;
      case 5:
        set_handler(idt, i, 1, exception5_stub);
        break;
      case 6:
        set_handler(idt, i, 1, exception6_stub);
        break;
      case 7:
        set_handler(idt, i, 1, exception7_stub);
        break;
      case 8:
        set_handler(idt, i, 1, exception8_stub);
        break;
      case 9:
        set_handler(idt, i, 1, exception9_stub);
        break;
      case 10:
        set_handler(idt, i, 1, exception10_stub);
        break;
      case 11:
        set_handler(idt, i, 1, exception11_stub);
        break;
      case 12:
        set_handler(idt, i, 1, exception12_stub);
        break;
      case 13:
        set_handler(idt, i, 1, exception13_stub);
        break;
      case 14:
        set_handler(idt, i, 1, exception14_stub);
        break;
      case 15: /* Interrupt 15 is reserved */
        set_handler(idt, i, 0, NULL);
        break;
      case 16:
        set_handler(idt, i, 1, exception16_stub);
        break;
      default:
        set_handler(idt, i, 1, interrupt_stub);
    }
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
