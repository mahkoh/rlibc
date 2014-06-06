#pragma once

#if defined(__x86_64__)
# include "arch/x86_64/stdint.h"
#else
# error "architecture not supported"
#endif
