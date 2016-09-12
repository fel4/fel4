export
SHARD_ROOT := $(realpath $(dir $(word $(words $(MAKEFILE_LIST)),$(MAKEFILE_LIST))))
SHARD_BUILD_ROOT ?= $(SHARD_ROOT)/build
SHARD_DOC_ROOT := $(SHARD_ROOT)/doc
SHARD_ISO_ROOT := $(SHARD_BUILD_ROOT)/iso
SHARD_KERNEL_ROOT := $(SHARD_ROOT)/kernel
SHARD_LIB_ROOT := $(SHARD_ROOT)/libs
SHARD_TOOL_ROOT := S(SHARD_ROOT)/tools

SHARD_ARCH ?= $(shell uname -m)
SHARD_BINARY_FORMAT := elf

SHARD_BUILD_ARCH_ROOT := $(SHARD_BUILD_ROOT)/arch/$(SHARD_ARCH)
SHARD_KERNEL_BINARY := $(SHARD_BUILD_ARCH_ROOT)/kernel.bin
SHARD_ISO := $(SHARD_ROOT)/shard.iso

include Tools.mk

.PHONY: all all-prep build-docs build-source clean install run

.DEFAULT: all

all: all-prep build-docs build-source

all-prep:
	@echo "Shard Project Root: $(SHARD_ROOT)"

build-docs:
	@$(MAKE) -C $(SHARD_DOC_ROOT)

build-source:
	@$(MAKE) -C $(SHARD_KERNEL_ROOT)

clean:
	@$(MAKE) -C $(SHARD_DOC_ROOT) clean
	@$(MAKE) -C $(SHARD_KERNEL_ROOT) clean

install:
	@$(MAKE) -C $(SHARD_KERNEL_ROOT) install

run: install
	@$(QEMU) -cdrom $(SHARD_ISO)

run-debug: install
	@$(QEMU) -d int -no-reboot -s -cdrom $(SHARD_ISO)
