export
SHARD_ROOT := $(realpath $(dir $(word $(words $(MAKEFILE_LIST)),$(MAKEFILE_LIST))))
SHARD_BUILD_ROOT ?= $(SHARD_ROOT)/build
SHARD_DOC_ROOT := $(SHARD_ROOT)/doc
SHARD_SOURCE_ROOT := $(SHARD_ROOT)/src

SHARD_ARCH ?= $(shell uname -m)
SHARD_BINARY_FORMAT := elf

SHARD_BUILD_ARCH_ROOT := $(SHARD_BUILD_ROOT)/arch/$(SHARD_ARCH)
SHARD_KERNEL_BINARY := $(SHARD_BUILD_ARCH_ROOT)/kernel.bin

include Tools.mk

.PHONY: all all-prep build-docs build-source clean

.DEFAULT: all

all: all-prep build-docs build-source

all-prep:
	@echo "Shard Project Root: $(SHARD_ROOT)"

build-docs:
	@$(MAKE) -C $(SHARD_DOC_ROOT)

build-source:
	@$(MAKE) -C $(SHARD_SOURCE_ROOT)

clean:
	@$(MAKE) -C $(SHARD_DOC_ROOT) clean
	@$(MAKE) -C $(SHARD_SOURCE_ROOT) clean
