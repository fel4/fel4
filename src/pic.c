#include <system.h>

#define PIC1  0x20
#define PIC2 0xA0

#define ICW1 0x11
#define ICW4 0x01

/* init_pics()
 * init the PICs and remap them
 */
void init_pics(int pic1, int pic2) {
  /* send ICW1 */
  outportb(PIC1, ICW1);
  outportb(PIC2, ICW1);
  
  /* send ICW2 */
  outportb(PIC1 + 1, pic1);   /* remap */
  outportb(PIC2 + 1, pic2);   /*  pics */
  
  /* send ICW3 */
  outportb(PIC1 + 1, 4);      /* IRQ2 -> connection to slave */
  outportb(PIC2 + 1, 2);
  
  /* send ICW4 */
  outportb(PIC1 + 1, ICW4);
  outportb(PIC2 + 1, ICW4);
  
  /* disable all IRQs */
  outportb(PIC1 + 1, 0x01);
}



