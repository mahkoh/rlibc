#include "libc.h"

int main(int argc, char const *argv[])
{
	puts("Hello, world!");
	//puts(getenv("HOME"));

	memcmp("bbb", "aaa", 3);
	return 0;
}
