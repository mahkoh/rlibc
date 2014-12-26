/// Avoid needless header complexity.
/// #include "libc.h"

#pragma once

typedef unsigned long size_t;
typedef signed long ssize_t;
typedef signed long off_t;

/* Types */
int isalnum(int);
int memcmp(const void *, const void *, size_t);


/* String manipulation */
char *strerror(int);
size_t strlen(const char *);

/* I/O */
int puts(const char *);

/* Filesystem */
int close(int);
ssize_t read(int, void *, size_t);
ssize_t write(int, const void *, size_t);
ssize_t pread(int, void *, size_t, off_t);
ssize_t pwrite(int, const void *, size_t, off_t);
off_t lseek(int, off_t, int);
int remove(const char *);
int rename(const char *, const char *);
int rmdir(const char*);
int unlink(const char *);

/* Environment */
const char *getarg(int index);
const char *getenv(int index);
extern int errno;

/* Math */
float floorf(float x);
float ceilf(float x);
float truncf(float x);
float roundf(float x);
float fmaf(float x, float y, float z);
float fmodf(float x, float y);
float fdimf(float x, float y);
float fmaxf(float x, float y);
float fminf(float x, float y);
float logf(float x);
float log2f(float x);
float log10f(float x);
float log1pf(float x);
float sqrtf(float x);
float cbrtf(float x);
float expf(float x);
float exp2f(float x);
float expm1f(float x);
float powf(float mant, float expo);
float sinf(float x);
float cosf(float x);
float tanf(float x);
float asinf(float x);
float acosf(float x);
float atanf(float x);
float acoshf(float x);
float asinhf(float x);
float atanhf(float x);
float coshf(float x);
float sinhf(float x);
float tanhf(float x);

double floor(double x);
double ceil(double x);
double trunc(double x);
double round(double x);
double fma(double x, double y, double z);
double fmod(double x, double y);
double fdim(double x, double y);
double fmax(double x, double y);
double fmin(double x, double y);
double log(double x);
double log2(double x);
double log10(double x);
double log1p(double x);
double sqrt(double x);
double cbrt(double x);
double exp(double x);
double exp2(double x);
double expm1(double x);
double pow(double mant, double expo);
double sin(double x);
double cos(double x);
double tan(double x);
double asin(double x);
double acos(double x);
double atan(double x);
double acosh(double x);
double asinh(double x);
double atanh(double x);
double cosh(double x);
double sinh(double x);
double tanh(double x);

double gauss(double x);
double erf(double x);
double erfc(double x);

/* Time */
long time(long*);

/* Dynamic loading */
void *dlopen(const char *filename, int flag);
char *dlerror(void);
void *dlsym(void *handle, const char *symbol);
int dlclose(void *handle);
int dladdr(void *addr, void *info);

/* Systemm calls */
int sys_exit(int);
int sys_write(int, const char *, unsigned long);
int sys_read(int, char *, unsigned long);
int sys_fork(void);
