#pragma once

#include "defines.h"
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
    Gamepad_Up = 1,
    Gamepad_Down = 2,
    Gamepad_Left = 4,
    Gamepad_Right = 8,
    Gamepad_North = 16,
    Gamepad_South = 32,
    Gamepad_East = 64,
    Gamepad_West = 128,
    Gamepad_LeftBumber = 256,
    Gamepad_RightBumper = 512,
    Gamepad_Start = 1024,
    Gamepad_Select = 2048,
    Gamepad_Disconnected = 4096
} gamepadstate_t;

WASM("gamepad", "get_state")
extern gamepadstate_t gamepad_GetState(uint8_t gamepad);

#ifdef __cplusplus
}
#endif