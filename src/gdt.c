#include <system.h>

gdt_entry_t gdt[5] = {
  /* null descriptor */
  {
    .limit  = 0,
    .base   = 0,
    .base1  = 0,
    .access = 0,
    .flags  = 0,
    .base2  = 0
  },
  /* kernel space code descriptor */
  {
    .limit  = 0xFFFF,
    .base   = 0x0,
    .base1  = 0x0,
    .access = 0x9A,
    .flags  = 0xCF,
    .base2  = 0x0
  },
  /* kernel space data descriptor */
  {
    .limit  = 0xFFFF,
    .base   = 0x0,
    .base1  = 0x0,
    .access = 0x92,
    .flags  = 0xCF,
    .base2  = 0x0
  },
  /* user space code descriptor */
  {
    .limit  = 0xFFFF,
    .base   = 0x0,
    .base1  = 0x0,
    .access = 0xFA,
    .flags  = 0xCF,
    .base2  = 0x0
  },
  /* user space data descriptor */
  {
    .limit  = 0xFFFF,
    .base   = 0x0,
    .base1  = 0x0,
    .access = 0xF2,
    .flags  = 0xCF,
    .base2  = 0x0
  }
};
