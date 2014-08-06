rlibc
=====

An implementation of libc and POSIX in Rust.

# Compiling

To build, you'll need Python, Make, and a copy of the Rust source.
* First copy the source of libcore into `./libcore`.
* Run `./fix_core.py` to disable split stacks in libcore.
* Optionally, create a `config.mk` file to specify custom tools and targets.
* Run `make`
