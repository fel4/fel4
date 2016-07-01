
# Toolchain 'triple'
SHARD_TOOLCHAIN_TARGET := $(SHARD_ARCH)-$(SHARD_BINARY_FORMAT)

# Absolute path to the linker
LD := /d/Binaries/Binutils/bin/$(SHARD_TOOLCHAIN_TARGET)-ld.exe

# Absolute path to the NASM executable
NASM := /d/Binaries/NASM/nasm.exe
