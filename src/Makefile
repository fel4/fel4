SHARD_KERNEL_ROOT := $(SHARD_SOURCE_ROOT)/kernel

.PHONY: build-kernel build-source

build-kernel:
	pushd $(SHARD_KERNEL_ROOT)
	$(MAKE)
	popd

build-source: build-kernel
