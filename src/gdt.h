/* GDT_H
 * Structure definition for an entry in the Global Descriptor Table
 */

#ifndef GDT_H
#define GDT_H

typedef struct {
  unsigned short limit;
  unsigned short base;
  unsigned char  base1;
  unsigned char  access;
  unsigned char  flags;
  unsigned char  base2;
} gdt_entry_t;

#endif /* GDT_H */
