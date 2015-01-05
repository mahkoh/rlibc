rlibc
=====

An implementation of libc and POSIX in Rust.

# Compiling

To build, you'll need Make, and a copy of the Rust source.
* First copy the source of libcore into `./libcore`.
* Optionally, create a `config.mk` file to specify custom tools and targets.
* Run `make`

# Coverage
rlibc currently supports part of C99 and POSIX, as well as some OS-specific functions.

* crt0 - mostly done
* mem - mostly done
* strings - mostly done
* math - mostly done
* printing - partial
* time - partial
* fs - partial
* mm - none
* environment - partial
* dl - almost none
* signals - almost none
* pthreads - NONE
* thread-local - NONE
* net - NONE
* atomics - NONE

# Targets

rlibc currently supports Linux and OS X on x86-64.
