-include ./config.mk

ARCH		   ?= x86_64
TARGET		   ?= x86_64-unknown-linux-gnu
RUST_ROOT	   ?= /usr
LLVM_ROOT	   ?= /usr
BINUTILS_ROOT  ?= /usr

BDIR			= target
TARGETDIR       = $(BDIR)/$(TARGET)

LD				= $(BINUTILS_ROOT)/bin/ld
CLANG			= $(LLVM_ROOT)/bin/clang
RUSTC           = $(RUST_ROOT)/bin/rustc

# add -L $(TARGETDIR)/deps for non-native platforms
RUSTCFLAGS		= --target $(TARGET) -O -Z no-landing-pads -C no-stack-check --out-dir $(TARGETDIR)
CLANGFLAGS		= -target $(TARGET) -I include/rlibc -nostdlib

.PHONY: all directories run clean

# Depend on librlibc.a so that cargo will create targets
all: directories $(TARGETDIR)/test

directories:
	mkdir -p $(BDIR)
	mkdir -p $(TARGETDIR)

$(TARGETDIR)/libcore.rlib: libcore/lib.rs
	$(RUSTC) $(RUSTCFLAGS) $<

$(TARGETDIR)/rlibc.o: src/lib.rs $(TARGETDIR)/libcore.rlib
	$(RUSTC) $(RUSTCFLAGS) src/lib.rs --emit obj -L $(TARGETDIR) --extern core=$(TARGETDIR)/libcore.rlib

$(TARGETDIR)/crt0.o: crt/$(TARGET)/crt0.s
	$(CLANG) $(CLANGFLAGS) -c $< -o $@

$(TARGETDIR)/test.o: test.c include/rlibc/libc.h
	$(CLANG) $(CLANGFLAGS) -c $< -o $@

$(TARGETDIR)/test: $(TARGETDIR)/test.o $(TARGETDIR)/crt0.o $(TARGETDIR)/rlibc.o $(TARGETDIR)/libcore.rlib $(RUST_ROOT)/lib/rustlib/$(TARGET)/lib/libcompiler-rt.a
	$(LD) -e start $^ -o $@

run: all
	$(TARGETDIR)/test

clean: directories
	rm -r $(BDIR)
