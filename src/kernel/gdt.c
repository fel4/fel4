#include <macros.h>
#include <system.h>

gdt_entry_t gdt[GDT_TBL_SIZ] = {};

extern tss_entry_t tss_table[CPU_COUNT];

/* this function correctly populates the GDT with the correct number of
 * entries. This exists as a temporary solution until a more mature system
 * exists to dynamically generate a valid GDT.
 */
void init_gdt() {

  int i = 0;
  unsigned long tss_addr = 0;
  /* null descriptor */
  gdt_entry_t null_desc = {
    .limit  = 0,
    .base   = 0,
    .base1  = 0,
    .access = 0,
    .flags  = 0,
    .base2  = 0
  };
  /* code descriptor */
  gdt_entry_t code_desc = {
    .limit  = 0xFFFF,
    .base   = 0x0,
    .base1  = 0x0,
    .access = 0x9A,
    .flags  = 0xCF,
    .base2  = 0x0
  };
  /* data descriptor */
  gdt_entry_t data_desc = {
    .limit  = 0xFFFF,
    .base   = 0x0,
    .base1  = 0x0,
    .access = 0x92,
    .flags  = 0xCF,
    .base2  = 0x0
  };

  /* setup the static gdt entries. */
  gdt[0] = null_desc;
  gdt[1] = code_desc;
  gdt[2] = data_desc;

  for (i = 3; i < GDT_TBL_SIZ; i++) {
    gdt[i].access = 0x89;
    gdt[i].limit = sizeof(tss_entry_t);
    gdt[i].flags = 0;
    gdt[i].flags = 3 << 6;
    tss_addr = (unsigned long) &(tss_table[i % 5]);
    gdt[i].base = (unsigned short) tss_addr & 0x0000FFFF;
    gdt[i].base1 = (unsigned char) (tss_addr >> 16) % 0xFF;
    gdt[i].base2 = (unsigned char) (tss_addr >> 24); 
  }
}
