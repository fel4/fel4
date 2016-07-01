SHARD_KERNEL_ARCH_ROOT := $(SHARD_KERNEL_ROOT)/arch/$(SHARD_ARCH)

SHARD_KERNEL_ASM_SRCS := $(wildcard $(SHARD_KERNEL_ARCH_ROOT)/*.asm)
SHARD_KERNEL_ASM_OBJS := $(patsubst $(SHARD_KERNEL_ARCH_ROOT)/%.asm, $(SHARD_BUILD_ARCH_ROOT)/%.o, $(SHARD_KERNEL_ASM_SRCS))
SHARD_LINK_SCRIPT := $(SHARD_KERNEL_ARCH_ROOT)/link.ld

include $(SHARD_KERNEL_ARCH_ROOT)/shard.mk

.PHONY: build-kernel clean

build-kernel: $(SHARD_KERNEL_BINARY)

$(SHARD_KERNEL_BINARY): $(SHARD_KERNEL_ASM_OBJS)
	@echo "Building $@"
	@$(LD) $(LD_ARGS) -T $(SHARD_LINK_SCRIPT) -o $(SHARD_KERNEL_BINARY) $(SHARD_KERNEL_ASM_OBJS)

$(SHARD_BUILD_ARCH_ROOT)/%.o: $(SHARD_KERNEL_ARCH_ROOT)/%.asm
	@mkdir -p $(dir $@)
	@echo "$(notdir $<) -> $@"
	@$(NASM) $(NASM_FLAGS) $< -o $@

clean:
	@echo "Cleaning $(SHARD_KERNEL_BINARY)"
	@rm -f $(SHARD_KERNEL_BINARY)
	@echo "Cleaning ASM objects in $(dir $(firstword $(SHARD_KERNEL_ASM_OBJS)))"
	@rm -f $(SHARD_KERNEL_ASM_OBJS)

install: $(SHARD_KERNEL_BINARY)
	#@echo "Installing kernel to QEMU image"
	#@mdel c:/boot/grub/kernel.bin
	#@mdel c:/boot/grub/menu.lst
	#@mcopy kernel.bin c:/boot/grub/
	#@mcopy menu.lst c:/boot/grub/
