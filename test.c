#include "libc.h"

int main(int argc, char const *argv[])
{
	puts("Hello, world!");
	puts(getenv(0));

	memcmp("bbb", "aaa", 3);
	return 0;
}
