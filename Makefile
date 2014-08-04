-include ./config.mk

ARCH			= x86_64
# TARGET			= x86_64-unknown-linux
TARGET			= x86_64-apple-darwin

RUST_ROOT	   ?= /usr/local
CARGO_ROOT	   ?= /usr/local
LLVM_ROOT	   ?= /usr
GCC_ROOT	   ?= /usr

BDIR			= ./target
MORESTACK		= morestack/$(ARCH)/morestack.S
TARGETDIR       = $(BDIR)/$(TARGET)

CLANG			= $(LLVM_ROOT)/bin/clang
RUSTC          ?= $(RUST_ROOT)/bin/rustc
CARGOBUILD		= $(CARGO_ROOT)/bin/cargo-build
CARGOCLEAN		= $(CARGO_ROOT)/bin/cargo-clean

# add -L $(TARGETDIR)/deps for non-native platforms
RUSTCFLAGS		= --target $(TARGET) -O -Z no-landing-pads --emit=obj
CLANGFLAGS		= -target $(TARGET) -nostdlib

.PHONY: all run clean

# Depend on librlibc.a so that cargo will create targets
all: $(TARGETDIR)/test

$(TARGETDIR)/deps/morestack.a: $(MORESTACK)
	$(CLANG) $(CLANGFLAGS) -c $< -o $@

# trick Cargo into setting everything up
$(TARGETDIR)/librlibc.rlib: src/lib.rs
	$(CARGOBUILD) $(RUSTCFLAGS)
$(TARGETDIR)/rlibc.o: $(TARGETDIR)/librlibc.rlib $(TARGETDIR)/deps/morestack.a
	$(RUSTC) $(RUSTCFLAGS) --out-dir $(TARGETDIR) src/lib.rs


$(TARGETDIR)/test.o: test.c
	$(CLANG) $(CLANGFLAGS) -c $< -o $@

$(TARGETDIR)/test: $(TARGETDIR)/rlibc.o $(TARGETDIR)/test.o
	$(CLANG) $(CLANGFLAGS) -e _start $^ -o $@

run: $(TARGETDIR)/test
	$(TARGETDIR)/test

clean:
	$(CARGOCLEAN)
