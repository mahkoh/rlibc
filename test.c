int isalnum(int);
int sys_exit(int);
int sys_write(int, const char *, unsigned long);
int sys_read(int, char *, unsigned long);
int sys_fork(void);
int unlink(const char *);
extern int errno;

int main(void)
{
	unlink("TESTFILE");
	return errno;
}
