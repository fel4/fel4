
SHARD_ROOT := $(realpath $(dir $(word $(words $(MAKEFILE_LIST)),$(MAKEFILE_LIST))))
SHARD_BUILD_ROOT ?= $(SHARD_ROOT)/build
SHARD_DOC_ROOT := $(SHARD_ROOT)/doc
SHARD_SOURCE_ROOT := $(SHARD_ROOT)/src

SHARD_ARCH ?= `uname -m`

SHARD_BUILD_ARCH_ROOT := $(SHARD_BUILD_ROOT)/arch/$(SHARD_ARCH)
SHARD_KERNEL_BINARY := $(SHARD_BUILD_ARCH_ROOT)/kernel.bin

include Tools.mk

.PHONY: all build-docs build-source clean

all: build-docs build-kernel
	@echo "Shard Project Root: $(SHARD_ROOT)"

build-docs:
	pushd $(SHARD_DOC_ROOT)
	$(MAKE)
	popd

build-source:
	pushd $(SHARD_SOURCE_ROOT)
	$(MAKE)
	popd
