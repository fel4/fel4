CFLAGS  := -ffreestanding -fno-stack-protector -fno-builtin -nostdinc -O -g -Wall -I. -m32
LDFLAGS := -nostdlib -Wl,-N -Wl,-Ttext -Wl,100000 -m elf-i386

SRCS := $(wildcard *.c)
OBJS := $(patsubst %.c, %.o, $(SRCS))
HEADERS := $(wildcard *.h)
ASM_OBJS := klib.o start.o isr_stubs.o



all: kernel.bin

kernel.bin: $(ASM_OBJS) $(OBJS)
	ld -T link.ld -m elf_i386 -o kernel.bin $^
	@echo Done!

%.o: %.c $(HEADERS)
	gcc $(CFLAGS) -c -o $@ $<

start.o: start.asm
	nasm -f elf -o start.o start.asm

klib.o: klib.asm
	nasm -f elf -o klib.o klib.asm

isr_stubs.o: isr_stubs.asm
	nasm -f elf -o isr_stubs.o isr_stubs.asm

clean:
	rm -f *.o *.bin

install: kernel.bin
	@echo "Installing kernel to QEMU image"
	@mdel c:/boot/grub/kernel.bin
	@mdel c:/boot/grub/menu.lst
	@mcopy kernel.bin c:/boot/grub/
	@mcopy ../../doc/menu.lst c:/boot/grub/
