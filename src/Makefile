all:
	rustc --opt-level 3 --emit=obj lib.rs
	ar rcs librlibc.a rlibc.o
	rm rlibc.o
