#pragma once

#include <common_types.h>
#include <defines.h>
#include <stdint.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

WASM("sprite", "set_sprite_tile")
extern void sprite_Set_Tile(uint8_t sprite, uint8_t tile);

WASM("sprite", "set_sprite_visible")
extern void sprite_Set_Visible(uint8_t sprite, bool visible);

WASM("sprite", "set_sprite_palette")
extern void sprite_Set_Palette(uint8_t sprite, uint8_t tile);

WASM("sprite", "get_sprite_position")
extern position_s16_t sprite_Get_Position(uint8_t sprite);

WASM("sprite", "get_sprite_position_x")
extern int16_t sprite_Get_Position_X(uint8_t sprite);

WASM("sprite", "get_sprite_position_y")
extern int16_t sprite_Get_Position_Y(uint8_t sprite);

WASM("sprite", "set_sprite_position")
extern void sprite_Set_Position(uint8_t sprite, position_s16_t pos);

WASM("sprite", "set_sprite_flip")
extern void sprite_Set_Flip(uint8_t sprite, bool flip_x, bool flip_y);

#ifdef __cplusplus
}
#endif
