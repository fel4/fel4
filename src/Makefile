export
SHARD_KERNEL_ROOT := $(SHARD_SOURCE_ROOT)/kernel

.PHONY: build-kernel build-source clean

build-kernel:
	@$(MAKE) -C $(SHARD_KERNEL_ROOT)

build-source: build-kernel

clean:
	@$(MAKE) -C $(SHARD_KERNEL_ROOT) clean
