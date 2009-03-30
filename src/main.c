//////////////////////////////////////////////////////////////////////
// File: main.c
#include <multiboot.h>
#include <system.h>

#define VERSION "version v0.00"

/* Check if the bit BIT in FLAGS is set. */
#define CHECK_FLAG(flags,bit)   ((flags) & (1 << (bit)))

/* This is a very simple main() function. All it does is print stuff
*  and then sit in an infinite loop. This will be like our 'idle'
*  loop */
void
cmain (unsigned long magic, unsigned long addr)
{
  multiboot_info_t *mbi;

  init_video();

  /* check multiboot information from bootloader. */
  if ( magic != MULTIBOOT_BOOTLOADER_MAGIC) {
    kprintf("multiboot magic: %d\nsaw magic: %d\n\n", MULTIBOOT_BOOTLOADER_MAGIC, magic);
  } else {
    kprintf("Saw valid multiboot magic\n\n");
  }
   
  /* set the address of the mbi struct to the address supplied by the bootloader. */
  mbi = (multiboot_info_t *) addr;

  kprintf("multiboot flags = 0x%x\n", mbi->flags);

  if ( CHECK_FLAG(mbi->flags, 0) ) {
    kprintf("mem_lower = %uKB, mem_upper %uKB\n",
            (unsigned) mbi->mem_lower, (unsigned) mbi->mem_upper);
  }

  /* print a welcome message. */
  kprintf("shard kernel\n%s\n\nWelcome to Shard!\n", VERSION);


  /* ...and leave this loop in. Note: there is an endless loop in
  *  'start.asm' also, if you accidentally delete this next line */
  for (;;);
}
//////////////////////////////////////////////////////////////////////                                                         
