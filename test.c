int isalnum(int);
int sys_exit(int);
int sys_write(int, const char *, unsigned long);
int sys_read(int, char *, unsigned long);

int main(void)
{
	char buf[20];
	sys_read(0, buf, 20);
	sys_write(1, "huhu\n", 5);
	sys_write(1, buf, 20);
	sys_exit(22);
	return isalnum('1');
}
