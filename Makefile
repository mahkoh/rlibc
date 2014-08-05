-include ./config.mk

ARCH		   ?= x86_64
# TARGET		   ?= x86_64-unknown-linux
TARGET		   ?= x86_64-apple-darwin

RUST_ROOT	   ?= /usr/local
LLVM_ROOT	   ?= /usr
BINUTILS_ROOT  ?= /usr

BDIR			= target
TARGETDIR       = $(BDIR)/$(TARGET)
MORESTACK		= morestack/$(ARCH)/morestack.S

LD				= $(BINUTILS_ROOT)/bin/ld
CLANG			= $(LLVM_ROOT)/bin/clang
RUSTC           = $(RUST_ROOT)/bin/rustc

# add -L $(TARGETDIR)/deps for non-native platforms
RUSTCFLAGS		= --target $(TARGET) -O -Z no-landing-pads --out-dir $(TARGETDIR)
CLANGFLAGS		= -target $(TARGET) -nostdlib

.PHONY: all run clean

# Depend on librlibc.a so that cargo will create targets
all: directories $(TARGETDIR)/test

directories:
	mkdir -p $(BDIR)
	mkdir -p $(TARGETDIR)

$(TARGETDIR)/libcore.rlib: libcore/lib.rs
	$(RUSTC) $(RUSTCFLAGS) $<

$(TARGETDIR)/rlibc.o: src/lib.rs $(TARGETDIR)/libcore.rlib
	$(RUSTC) $(RUSTCFLAGS) src/lib.rs --emit=obj -L $(TARGETDIR) --extern core=$(TARGETDIR)/libcore.rlib

$(TARGETDIR)/test.o: test.c
	$(CLANG) $(CLANGFLAGS) -c $< -o $@

$(TARGETDIR)/test: $(TARGETDIR)/rlibc.o $(TARGETDIR)/test.o
	$(LD) -e _start $^ -o $@

run: all
	$(TARGETDIR)/test

clean:
	rm -r $(BDIR)
