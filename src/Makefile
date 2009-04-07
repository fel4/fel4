CFLAGS  := -fno-stack-protector -fno-builtin -nostdinc -O -g -Wall -I. -m32
#CFLAGS  := -fno-builtin -nostdinc -O -g -Wall -I. -m32
LDFLAGS := -nostdlib -Wl,-N -Wl,-Ttext -Wl,100000 -m elf-i386

SRCS := $(wildcard *.c)
OBJS := $(patsubst %.c, %.o, $(SRCS))

all: kernel.bin

kernel.bin: start.o klib.o $(OBJS)
	ld -T link.ld -m elf_i386 -o kernel.bin $^
	@echo Done!

%.o: %.c system.h
	gcc $(CFLAGS) -c -o $@ $<

#kernel2.bin: start.asm main.c scrn.c start.o
#	gcc -o kernel.bin $(CFLAGS) start.o main.c scrn.c $(LDFLAGS)
#

start.o: start.asm
	nasm -f elf -o start.o start.asm

klib.o: klib.asm
	nasm -f elf -o klib.o klib.asm

clean:
	rm -f *.o *.bin

install: kernel.bin
	@echo "Installing kernel to QEMU image"
	@mdel c:/boot/grub/kernel.bin
	@mcopy kernel.bin c:/boot/grub/
