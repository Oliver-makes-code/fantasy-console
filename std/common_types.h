#pragma once

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    int16_t x;
    int16_t y;
} position_s16_t;

typedef struct {
    uint8_t x;
    uint8_t y;
} position_u8_t;

#ifdef __cplusplus
}
#endif
