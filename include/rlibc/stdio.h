struct FILE;

extern FILE __stdin;
extern FILE __stdout;
extern FILE __stderr;

#define stdin  &__stdin
#define stdout &__stdout
#define stderr &__stderr

#define _IOFBF 0
#define _IOLBF 1
#define _IONBF 2

#define BUFSIZ 8192

#define EOF (-1)

#define FOPEN_MAX 16
#define FILENAME_MAX 4096
#define L_tmpnam 20

#define SEEK_SET 0
#define SEEK_CUR 1
#define SEEK_END 2

#define TMP_MAX 238328
