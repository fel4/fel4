/* ata_driver.c - ATA controller / disk routines. */

#include <macros.h>
#include <system.h>

/* this function uses lba28/48 via pio for block device access. */
unsigned char *access_block(int controller, int drive, unsigned long long addr, unsigned char* buffer, int rw_mode, int lba48_mode) {

  int idx = 0;
  unsigned short tmpword = 0;

  /* initialization. */
  outportb(controller + 1, 0x00);

  /* sector count. */
  if ( lba48_mode ) {
    outportb(controller + 2, 0x00);
  }
  outportb(controller + 2, 0x01);

  /* write in the (low)24 / 48 bits of the address, 8 bits at a time. */
  if ( lba48_mode ) {
    outportb(controller + 3, (unsigned char)(addr >> 24));
  }
  outportb(controller + 3, (unsigned char)addr);
  if ( lba48_mode ) {
    outportb(controller + 4, (unsigned char)(addr >> 32));
  }
  outportb(controller + 4, (unsigned char)(addr >> 8));
  if ( lba48_mode ) {
    outportb(controller + 5, (unsigned char)(addr >> 40));
  }
  outportb(controller + 5, (unsigned char)(addr >> 16));

  /* send the drive id, some magic, and and remaining address bits ( lba 28 only ) */
  if ( lba48_mode ) {
    outportb(controller + 6, 0x40 | ( drive << 4 ));
  } else {
    outportb(controller + 6, 0xE0 | ( drive << 4 ) | ((addr >> 24) & 0x0F));
  }

  /* send the command to the controller. */
  if ( lba48_mode ) {
    outportb(controller + 7, rw_mode + 4);
  } else {
    outportb(controller + 7, rw_mode);
  }

  /* wait for the drive to be ready */
  while (!(inportb(controller + 7) & 0x08)) {}

  if ( rw_mode == READ_MODE ) {
    for (idx = 0; idx < 256; idx++) {
      tmpword = inportw(0x1F0);
      buffer[idx * 2] = (unsigned char)tmpword;
      buffer[idx * 2 + 1] = (unsigned char)(tmpword >> 8);
    }
  } else {
    for (idx = 0; idx < 256; idx++) {
      tmpword = buffer[8 + idx * 2] | (buffer[8 + idx * 2 + 1] << 8);
      outportw(0x1F0, tmpword);
    }
  }
  return buffer;
}
