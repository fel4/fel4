/* 
 * macros.h -- preprocessor macros for shard
 */

#ifndef MACROS_H
#define MACROS_H

#define NULL (void*)0

/* Check if the bit BIT in FLAGS is set. */
#define CHECK_FLAG(flags,bit)   ((flags) & (1 << (bit)))

/* The magic number for the Multiboot header. */
#define MULTIBOOT_HEADER_MAGIC          0x1BADB002

/* The flags for the Multiboot header. */
#ifdef __ELF__
  # define MULTIBOOT_HEADER_FLAGS         0x00000003
#else
  # define MULTIBOOT_HEADER_FLAGS         0x00010003
#endif

/* The magic number passed by a Multiboot-compliant boot loader. */
#define MULTIBOOT_BOOTLOADER_MAGIC      0x2BADB002

/* C symbol format. HAVE_ASM_USCORE is defined by configure. */
#ifdef HAVE_ASM_USCORE
  # define EXT_C(sym)                     _ ## sym
#else
  # define EXT_C(sym)                     sym
#endif

/* System limits are currently hardcoded here until a better way becomes
 * feasible, like being able to detect and configure that stuff at boot time.
 */

/* The size of our stack (16KB). */
#define STACK_SIZE                      0x4000

#define CPU_COUNT 8

#define GDT_TBL_SIZ 3 + CPU_COUNT

#define NUM_INTERRUPTS 256

#define CTRL1_PORT_BASE 0x1F0
#define CTRL2_PORT_BASE 0x170

#define READ_MODE 0x20
#define WRITE_MODE 0x30

#define LBA28_MODE 0
#define LBA48_MODE 1


#endif /* MACROS_H */

