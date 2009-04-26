#include <system.h>

#define RELEASE_MASK 0x80

typedef enum { 
  KBD_ERROR, KBD_ESC, KBD_ONE, KBD_TWO, KBD_THREE, KBD_FOUR, KBD_FIVE, KBD_SIX, 
  KBD_SEVEN, KBD_EIGHT, KBD_NINE, KBD_ZERO, KBD_MINUS, KBD_EQUAL, KBD_BACKSPACE, 
  KBD_TAB, KBD_Q, KBD_W, KBD_E, KBD_R, KBD_T, KBD_Y, KBD_U, KBD_I, KBD_O, KBD_P, 
  KBD_OPEN_BRACKET, KBD_CLOSE_BRACKET, KBD_ENTER, KBD_L_CTRL, KBD_A, KBD_S, 
  KBD_D, KBD_F, KBD_G, KBD_H, KBD_J, KBD_K, KBD_L, KBD_SEMI, KBD_APOS, KBD_BACKTICK, 
  KBD_LSHIFT, KBD_BACKSLASH, KBD_Z, KBD_X, KBD_C, KBD_V, KBD_B, KBD_N, KBD_M, 
  KBD_COMMA, KBD_DOT, KBD_SLASH, KBD_RSHIFT, KBD_KP_STAR, KBD_LALT, KBD_SPACE, 
  KBD_CAPSLOCK, KBD_F1, KBD_F2, KBD_F3, KBD_F4, KBD_F5, KBD_F6, KBD_F7, KBD_F8, 
  KBD_F9, KBD_F10, KBD_NUMLOCK, KBD_SCRLLOCK, KBD_HOME, KBD_UP, KBD_PGUP, KBD_KP_MINUS, 
  KBD_LEFT, KBD_KP_FIVE, KBD_RIGHT, KBD_KP_PLUS, KBD_END, KBD_DOWN, KBD_PGDN, KBD_INS, KBD_DEL
} KBD_CODE;

static char code_to_char(KBD_CODE code) {
  switch (code) {
    case KBD_ONE: return '1';
    case KBD_TWO: return '2';
    case KBD_THREE: return '3';
    case KBD_FOUR: return '4';
    case KBD_FIVE: return '5';
    case KBD_SIX: return '6';
    case KBD_SEVEN: return '7';
    case KBD_EIGHT: return '8';
    case KBD_NINE: return '9';
    case KBD_ZERO: return '0';
    case KBD_Q: return 'q';
    case KBD_W: return 'w';
    case KBD_E: return 'e';
    case KBD_R: return 'r';
    case KBD_T: return 't';
    case KBD_Y: return 'y';
    case KBD_U: return 'u';
    case KBD_I: return 'i';
    case KBD_O: return 'o';
    case KBD_P: return 'p';
    case KBD_ENTER: return '\n';
    case KBD_A: return 'a';
    case KBD_S: return 's';
    case KBD_D: return 'd';
    case KBD_F: return 'f';
    case KBD_G: return 'g';
    case KBD_H: return 'h';
    case KBD_J: return 'j';
    case KBD_K: return 'k';
    case KBD_L: return 'l';
    case KBD_SEMI: return ';';
    case KBD_APOS: return '\'';
    case KBD_BACKTICK: return '`';
    case KBD_BACKSLASH: return '\\';
    case KBD_Z: return 'z';
    case KBD_X: return 'x';
    case KBD_C: return 'c';
    case KBD_V: return 'v';
    case KBD_B: return 'b';
    case KBD_N: return 'n';
    case KBD_M: return 'm';
    case KBD_COMMA: return ',';
    case KBD_DOT: return '.';
    case KBD_SLASH: return '/';
    case KBD_SPACE: return ' ';
    default:  return '\0';
  }
}

void handle_kbd_event(void) {

  unsigned char scancode;

  scancode = inportb(0x60);

  /*kprintf("saw scancode %x\n", scancode);*/
  if ( ! (scancode & RELEASE_MASK) ) {
    kprintf("%c", code_to_char(scancode));
  }
}
