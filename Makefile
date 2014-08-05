-include ./config.mk

ARCH		   ?= x86_64
# TARGET		   ?= x86_64-unknown-linux
TARGET		   ?= x86_64-apple-darwin

RUST_ROOT	   ?= /usr/local
LLVM_ROOT	   ?= /usr
GCC_ROOT	   ?= /usr

BDIR			= target
TARGETDIR       = $(BDIR)/$(TARGET)
MORESTACK		= morestack/$(ARCH)/morestack.S

LD				= $(GCC_ROOT)/bin/ld
CLANG			= $(LLVM_ROOT)/bin/clang
RUSTC          ?= $(RUST_ROOT)/bin/rustc

# add -L $(TARGETDIR)/deps for non-native platforms
RUSTCFLAGS		= --target $(TARGET) -O -Z no-landing-pads --out-dir $(TARGETDIR)
CLANGFLAGS		= -target $(TARGET) -nostdlib

.PHONY: all run clean

# Depend on librlibc.a so that cargo will create targets
all: directories $(TARGETDIR)/test

directories:
	mkdir -p $(BDIR)
	mkdir -p $(TARGETDIR)

$(TARGETDIR)/morestack.o: $(MORESTACK)
	$(CLANG) $(CLANGFLAGS) -c $< -o $@

$(TARGETDIR)/libcore.rlib: libcore/lib.rs
	$(RUSTC) $(RUSTCFLAGS) $<

$(TARGETDIR)/rlibc.o: src/lib.rs $(TARGETDIR)/libcore.rlib $(TARGETDIR)/morestack.o
	$(RUSTC) $(RUSTCFLAGS) src/lib.rs --emit=obj -L $(TARGETDIR) --extern core=$(TARGETDIR)/libcore.rlib

# $(TARGETDIR)/rlibc.o: $(TARGETDIR)/librlibc.rlib $(TARGETDIR)/deps/morestack.a
# 	$(RUSTC) $(RUSTCFLAGS) --out-dir $(TARGETDIR) src/lib.rs


$(TARGETDIR)/test.o: test.c
	$(CLANG) $(CLANGFLAGS) -c $< -o $@

$(TARGETDIR)/test: $(TARGETDIR)/rlibc.o $(TARGETDIR)/test.o
	$(CLANG) $(CLANGFLAGS) -e _start $^ -o $@

run: all
	$(TARGETDIR)/test

clean:
	$(CARGOCLEAN)
