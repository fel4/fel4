/* 
 * arch_init.c - Final architecture initialization 
 */
#include <multiboot.h>
#include <macros.h>
#include <system.h>

extern gdt_entry_t gdt[GDT_TBL_SIZ];
extern idt_entry_t idt[NUM_INTERRUPTS];

/* assembly function prototypes ( defined in klib.asm ) */
extern void set_gdt(unsigned long gdt_addr, unsigned int gdt_length);
extern void set_idt(unsigned long idt_addr, unsigned int idt_length);
extern void enable_interrupts();
extern void disable_interrupts();
extern void reload_segments();

/*
 * This function performs some final arch specific initialization
 * before kernel_main() gets called -- our real main() function.
 */
void arch_init (unsigned long magic, unsigned long addr)
{
  multiboot_info_t *mbi;
  gdt_entry_t *new_gdt;
  idt_entry_t *new_idt;

  init_video();

  /* check multiboot information from bootloader. */
  if ( magic != MULTIBOOT_BOOTLOADER_MAGIC) {
    kprintf("multiboot magic: %d\nsaw magic: %d\n\n", MULTIBOOT_BOOTLOADER_MAGIC, magic);
  } else {
    kprintf("Saw valid multiboot magic\n\n");
  }
   
  /* set the address of the mbi struct to the address supplied by the bootloader. */
  mbi = (multiboot_info_t *) addr;

  /*
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
  */

  /* read aout or elf header info */
  /*
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
  }
  */

  /* setup the global descriptor table. */
  kprintf("GDT table size: %d, GDT entry size: %d\n", sizeof(gdt), sizeof(gdt_entry_t));
  kprintf("Attempting to setup the GDT\n");
  init_gdt();
  
  /* move the GDT to its real location.*/
  kprintf("moving the GDT to its permanent location\n");
  new_gdt = (gdt_entry_t*)memcpy((unsigned char*)0x800, (const unsigned char*)gdt, sizeof(gdt));
  set_gdt((unsigned long)new_gdt, sizeof(gdt));

 /* reload the segment registers to point to the new descriptors. */
  reload_segments();


  /* setup interrupts */
  kprintf("Attempting to setup the IDT\n");
  init_idt(); // setup the IDT table.
  new_idt = (idt_entry_t*)memcpy((unsigned char*)0x0000, (const unsigned char*)idt, sizeof(idt));
  set_idt((unsigned long)new_idt, sizeof(idt));

  /* reload the segment registers to point to the new descriptors. */
  reload_segments();

  enable_interrupts();
  kprintf("\nenabled interrupts.\n");
}
