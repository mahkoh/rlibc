.text
.global start
start:
	xor %rbp,%rbp   /* rbp:undefined -> mark as zero 0 (ABI) */
	pop %rdi        /* 1nd arg: argc */
	mov %rsp,%rsi   /* 2rd arg: argv */
	andq $-16,%rsp  /* align stack pointer */
	call __libc_start_main /* musl init will run the program */
1:	hlt
	jmp 1b
