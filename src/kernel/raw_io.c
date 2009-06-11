/* raw_io.c -- low-level io routines ( {in,out}port{b,w}, etc. ) */

/* these functions wrap the in{b,w} assembly calls allowing c functions
   to read data from ports. */
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


/* these functions operate in a manner similar to inport{b,w}, except
   that they write data instead of reading it. */
void outportb (unsigned short _port, unsigned char _data)
{
  __asm__ __volatile__ ("outb %1, %0" : : "dN" (_port), "a" (_data));
}

void outportw (unsigned short _port, unsigned short _data)
{
  __asm__ __volatile__ ("outw %1, %0" : : "dN" (_port), "a" (_data));
}
