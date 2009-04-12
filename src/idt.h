/* idt.h */

#ifndef IDT_H
#define IDT_H

#define NUM_INTERRUPTS 256

typedef struct {
   unsigned short offset_1; // offset bits 0..15
   unsigned short selector; // a code segment selector in GDT or LDT
   unsigned char zero;      // unused, set to 0
   unsigned char type_attr; // type and attributes, see below
   unsigned short offset_2; // offset bits 16..31
} idt_entry_t;

extern void init_idt(idt_entry_t table[], void (*def_handler)(void), int count);

extern void set_handler(idt_entry_t table[], int inter_num, void (*handler)(void));

#endif /* IDT_H */
