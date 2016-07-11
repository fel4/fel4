SHARD_KERNEL_ARCH_ROOT := $(SHARD_KERNEL_ROOT)/arch/$(SHARD_ARCH)
SHARD_KERNEL_RUST_ROOT := $(SHARD_KERNEL_ROOT)/src

SHARD_KERNEL_ASM_SRCS := $(wildcard $(SHARD_KERNEL_ARCH_ROOT)/*.asm)
SHARD_KERNEL_ASM_OBJS := $(patsubst $(SHARD_KERNEL_ARCH_ROOT)/%.asm, $(SHARD_BUILD_ARCH_ROOT)/%.o, $(SHARD_KERNEL_ASM_SRCS))
SHARD_KERNEL_RUST_SRCS := $(shell find $(SHARD_BUILD_ARCH_ROOT) -name *.rs)
SHARD_LINK_SCRIPT := $(SHARD_KERNEL_ARCH_ROOT)/link.ld

SHARD_KERNEL_ISO_PATH := $(SHARD_ISO_ROOT)/boot/kernel.bin
SHARD_GRUB_CONFIG := $(SHARD_ISO_ROOT)/boot/grub/grub.cfg

SHARD_TARGET := x86_64-unknown-linux-gnu
SHARD_RUST_STATICLIB := $(SHARD_KERNEL_ROOT)/target/$(SHARD_TARGET)/debug/libshard_kernel.a

include $(SHARD_KERNEL_ARCH_ROOT)/shard.mk

.PHONY: build-image build-kernel clean install $(SHARD_RUST_STATICLIB)

build-image: $(SHARD_KERNEL_ISO_PATH) $(SHARD_GRUB_CONFIG)
	@echo "Building ISO image"
	@grub-mkrescue -o $(SHARD_ISO) $(SHARD_ISO_ROOT) 2> /dev/null

build-kernel: $(SHARD_KERNEL_BINARY)

$(SHARD_KERNEL_BINARY): $(SHARD_KERNEL_ASM_OBJS) $(SHARD_RUST_STATICLIB)
	@echo "Building $@"
	@$(LD) $(LD_ARGS) -T $(SHARD_LINK_SCRIPT) -o $(SHARD_KERNEL_BINARY) $(SHARD_KERNEL_ASM_OBJS) $(SHARD_RUST_STATICLIB)

$(SHARD_BUILD_ARCH_ROOT)/%.o: $(SHARD_KERNEL_ARCH_ROOT)/%.asm
	@mkdir -p $(dir $@)
	@echo "$(notdir $<) -> $@"
	@$(NASM) $(NASM_FLAGS) $< -o $@

$(SHARD_RUST_STATICLIB): $(SHARD_KERNEL_RUST_SRCS)
	@echo "Building Shard Cargo project"
	@cargo build --target $(SHARD_TARGET)

$(SHARD_GRUB_CONFIG): $(SHARD_KERNEL_ARCH_ROOT)/grub.cfg
	@echo "Configuring ISO boot settings"
	@mkdir -p $(SHARD_ISO_ROOT)/boot/grub
	@cp $(SHARD_KERNEL_ARCH_ROOT)/grub.cfg $(SHARD_GRUB_CONFIG)

$(SHARD_KERNEL_ISO_PATH): $(SHARD_KERNEL_BINARY)
	@echo "Installing kernel to QEMU image"
	@mkdir -p $(SHARD_ISO_ROOT)/boot
	@cp $(SHARD_KERNEL_BINARY) $(SHARD_KERNEL_ISO_PATH)

clean:
	@echo "Cleaning $(SHARD_KERNEL_BINARY)"
	@rm -f $(SHARD_KERNEL_BINARY)
	@echo "Cleaning ASM objects in $(dir $(firstword $(SHARD_KERNEL_ASM_OBJS)))"
	@rm -f $(SHARD_KERNEL_ASM_OBJS)
	@echo "Running Cargo clean"
	@cargo clean

install: build-image
