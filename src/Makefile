all:
	rustc -O --emit=obj lib.rs
	ar rcs librlibc.a rlibc.o
	rm rlibc.o
