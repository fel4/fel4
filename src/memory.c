/* memory.c
 * memory manipulation routines for the shard kernel
 */

#include <system.h>

/* some convenient functions - as we don't have libc, we must do
   everything ourselves */

unsigned char *memcpy(unsigned char *dest, const unsigned char *src, int count)
{
  int i;
  for (i = 0; i < count; i++) {
    dest[i] = src[i];
  }
  return dest;
}

unsigned char *memset(unsigned char *dest, unsigned char val, int count)
{
  int i;
  for (i = 0; i < count; i++) {
    dest[i] = val;
  }
  return dest;
}

unsigned short *memsetw(unsigned short *dest, unsigned short val, int count)
{
  int i;
  for (i = 0; i < count; i++) {
    dest[i] = val;
  }
  return dest;
}
