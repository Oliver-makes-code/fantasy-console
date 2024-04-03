#pragma once

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef unsigned long size_t;
typedef signed long ptrdiff_t;

void *malloc(size_t size);

void free(void *ptr);

#ifdef __cplusplus
}
#endif
