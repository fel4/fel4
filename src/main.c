//////////////////////////////////////////////////////////////////////
// File: main.c
#include <gdt.h>
#include <multiboot.h>
#include <system.h>

#define VERSION "version v0.00"

extern gdt_entry_t gdt[5];

/* Check if the bit BIT in FLAGS is set. */
#define CHECK_FLAG(flags,bit)   ((flags) & (1 << (bit)))

/* function prototype for set_gdt ( defined in start.asm ) */
void set_gdt(unsigned long gdt_addr, unsigned int gdt_length);

static void dump_memory_maps(unsigned long addr, unsigned int count) {

  int i = 0;
  memory_map_t *current = (memory_map_t *)addr;

  for ( i = 0; i < count; i++ ) {
    kprintf("map is %s\n", ( current->type == 1 ) ? "available" : "unavailble");
    kprintf("\tbase_low = %d, base_high = %d\n\tlength_low = %d,length_high = %d\n",
            current->base_addr_low, current->base_addr_high,
            current->length_low, current->length_high);
    current++;
  }
}

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

  if ( CHECK_FLAG(mbi->flags, 2) ) {
    kprintf("cmdline = %s\n", mbi->cmdline);
  } else {
    kprintf("no cmdline supplied\n");
  }

  /* read aout or elf header info */
  if ( CHECK_FLAG(mbi->flags, 4) &&
       CHECK_FLAG(mbi->flags, 5) ) {
    kprintf("ERROR: flags 4 & 5 are mutually exclusive!\n");
  } else if ( CHECK_FLAG(mbi->flags, 4) ) {
    kprintf("aout info = \n\ttabsize: %d\n\tstrsize = %d\n\taddr = 0x%x\n", 
            mbi->u.aout_sym.tabsize, mbi->u.aout_sym.strsize, mbi->u.aout_sym.addr);
  } else if ( CHECK_FLAG(mbi->flags, 5) ) {
    kprintf("elf info = \n\tnum =  %d\n\tsize = %d\n\taddr = 0x%x\n\tshndx = %d\n",
            mbi->u.elf_sec.num, mbi->u.elf_sec.size, mbi->u.elf_sec.addr, mbi->u.elf_sec.shndx);
  }

  if ( CHECK_FLAG(mbi->flags, 6) ) {
    kprintf("mmap_length = %d\nmmap_addr = 0x%u\n",
            mbi->mmap_length, mbi->mmap_addr);
    kprintf("found %d memory maps.\n\n", mbi->mmap_length / sizeof(memory_map_t) );
    /*dump_memory_maps( mbi->mmap_addr, mbi->mmap_length / sizeof(memory_map_t) );*/
  }

  /* setup the global descriptor table. */
  kprintf("Attempting to setup the GDT\n");
  set_gdt((unsigned long)gdt, sizeof(gdt));

  /* print a welcome message. */
  kprintf("shard kernel\n%s\n\nWelcome to Shard!\n", VERSION);


  /* ...and leave this loop in. Note: there is an endless loop in
  *  'start.asm' also, if you accidentally delete this next line */
  for (;;);
}
//////////////////////////////////////////////////////////////////////                                                         
