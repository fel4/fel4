/* 
 * main.c - C entry point for kernel
 */
#include <system.h>

#define VERSION "version v0.00"

/* This is a very simple main() function. All it does is print stuff
*  and then sit in an infinite loop. This will be like our 'idle'
*  loop */
void kernel_main (void)
{

  /* print a welcome message. */
  kprintf("\nshard kernel\n%s\n\nWelcome to Shard!\n", VERSION);

  /* ...and leave this loop in. Note: there is an endless loop in
   *  'start.asm' also, if you accidentally delete this next line */
  for (;;);
}
