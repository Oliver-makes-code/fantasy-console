#pragma once

#include "defines.h"
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
    Up = 1,
    Down = 2,
    Left = 4,
    Right = 8,
    North = 16,
    South = 32,
    East = 64,
    West = 128,
    LeftBumber = 256,
    RightBumper = 512,
    Start = 1024,
    Select = 2048,
    Disconnected = 4096
} gamepadstate_t;

WASM("gamepad", "get_gamepad_state")
extern gamepadstate_t gamepad_GetState(uint8_t gamepad);

#ifdef __cplusplus
}
#endif