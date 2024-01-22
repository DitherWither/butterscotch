# Nuke built-in rules and variables.
override MAKEFLAGS += -rR

ifeq ($(RUST_PROFILE),)
    override RUST_PROFILE := dev
endif

override RUST_PROFILE_SUBDIR := $(RUST_PROFILE)
ifeq ($(RUST_PROFILE),dev)
    override RUST_PROFILE_SUBDIR := debug
endif

# Default target.
.PHONY: all
all:
	cargo build --profile $(RUST_PROFILE)
	cp target/x86_64-butterscotch_kernel/$(RUST_PROFILE_SUBDIR)/butterscotch_kernel kernel

# Remove object files and the final executable.
.PHONY: clean
clean:
	cargo clean
	rm -rf kernel

.PHONY: distclean
distclean: clean
