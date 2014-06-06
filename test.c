int isalnum(int);
int sys_exit(int);
int sys_write(int, const char *, unsigned long);
int sys_read(int, char *, unsigned long);
int sys_fork(void);
int unlink(const char *);
extern int errno;
int remove(const char *);
int rename(const char *, const char *);
unsigned long strlen(const char *);

int main(void)
{
	return strlen("huhu");
}
