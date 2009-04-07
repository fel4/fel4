/* idt.h */

#ifndef IDT_H
#define IDT_H

typedef struct {
   unsigned short offset_1; // offset bits 0..15
   unsigned short selector; // a code segment selector in GDT or LDT
   unsigned char zero;      // unused, set to 0
   unsigned char type_attr; // type and attributes, see below
   unsigned short offset_2; // offset bits 16..31
} idt_entry_t;

#endif /* IDT_H */
