#include <macros.h>
#include <system.h>

tss_entry_t tss_table[CPU_COUNT] = {};

void init_tss() {
  int i = 0;

  tss_entry_t empty = { 0 };

  for ( i = 0; i < CPU_COUNT; i++ ) {
    tss_table[i] = empty;
  }
}
