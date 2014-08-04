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

typedef unsigned long size_t;

char *strerror(int);
size_t strlen(const char *);
int puts(const char *);
int memcmp(const void *, const void *, size_t);


int main(void)
{
	return memcmp("bbb", "aaa", 3);
}
