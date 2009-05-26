/* string.c
 * string manipulation functions for the shard kernel
 */

#include <system.h>

int strlen(const char *str)
{
  int i;
  for (i=0;;i++) {
    if (str[i] == '\0') {
      return i;
    }
  }
}

/* We can use this for reading from the I/O ports to get data from
*  devices such as the keyboard. We are using what is called 'inline
*  assembly' in these routines to actually do the work. [XXX I still
*  have to add devices to the tutorial] */
unsigned char inportb (unsigned short _port)
{
  unsigned char rv;
  __asm__ __volatile__ ("inb %1, %0" : "=a" (rv) : "dN" (_port));
  return rv;
}

unsigned short inportw (unsigned short _port)
{
  unsigned short rv;
  __asm__ __volatile__ ("inw %1, %0" : "=a" (rv) : "dN" (_port));
  return rv;
}


/* We can use this to write to I/O ports to send bytes to
*  devices. Again, we use some inline assembly for the stuff that
*  simply cannot be done in C */
void outportb (unsigned short _port, unsigned char _data)
{
  __asm__ __volatile__ ("outb %1, %0" : : "dN" (_port), "a" (_data));
}

void outportw (unsigned short _port, unsigned short _data)
{
  __asm__ __volatile__ ("outw %1, %0" : : "dN" (_port), "a" (_data));
}


/* Convert the integer D to a string and save the string in BUF. If
BASE is equal to 'd', interpret that D is decimal, and if BASE is
]equal to 'x', interpret that D is hexadecimal. */
static void
itoa (char *buf, int base, int d) {
  char *p = buf;
  char *p1, *p2;
  unsigned long ud = d;
  int divisor = 10;
  
  /* If %d is specified and D is minus, put `-' in the head. */
  if (base == 'd' && d < 0) {
    *p++ = '-';
    buf++;
    ud = -d;
  }
  else if (base == 'x') {
    divisor = 16;
  }
  /* Divide UD by DIVISOR until UD == 0. */
  do {
    int remainder = ud % divisor;
    
    *p++ = (remainder < 10) ? remainder + '0' : remainder + 'a' - 10;
  }  while (ud /= divisor);                                                                                                                                                                                                                                  
  /* Terminate BUF. */
  *p = 0;
  
  /* Reverse BUF. */
  p1 = buf;
  p2 = p - 1;
  while (p1 < p2) {
    char tmp = *p1;
    *p1 = *p2;
    *p2 = tmp;
    p1++;
    p2--;
  }
}

/* Format a string and print it on the screen, just like the libc
function printf. */
void kprintf (const char *format, ...) {
  char **arg = (char **) &format;
  int c;
  char buf[20];
  
  arg++;
  
  while ((c = *format++) != 0) {
    if (c != '%') {
      putch (c);
    } else {
      char *p;
      
      c = *format++;
      switch (c) {
        case 'd':
        case 'u':
        case 'x':
          itoa (buf, c, *((int *) arg++));
          p = buf;
          goto string;
          break;
          
        case 's':
          p = *arg++;
          if (! p) {
            p = "(null)";
          }
          
        string:
          while (*p) {
            putch (*p++);
          }
          break;
            
        default:
          putch (*((int *) arg++));
          break;
      }
    }
  }
}

