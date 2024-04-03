#pragma once

#include <common_types.h>
#include <defines.h>
#include <stdint.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

WASM("tile", "write_palette")
extern void tile_WritePalette(uint8_t palette, uint32_t color);

WASM("tile", "write_tile")
extern void tile_WriteTile(uint8_t tile_number, const uint8_t *tile);

WASM("tile", "set_background_tile")
extern void tile_Set_BackgroundTile(uint8_t background, position_u8_t pos, uint8_t tile);


WASM("tile", "set_background_visible")
extern void tile_Set_BackgroundVisible(uint8_t background, bool visible);


WASM("tile", "set_background_palette")
extern void tile_Set_BackgroundPalette(uint8_t palette);


WASM("tile", "set_background_tile_palette")
extern void tile_Set_BackgroundTilePalette(uint8_t background, position_u8_t pos, uint8_t palette);


WASM("tile", "set_background_scroll_pre")
extern void tile_Set_BackgroundScroll_Pre(uint8_t background, position_s16_t pos);

WASM("tile", "set_background_scroll_pre_x")
extern void tile_Set_BackgroundScroll_Pre_X(uint8_t background, int16_t x);

WASM("tile", "set_background_scroll_pre_y")
extern void tile_Set_BackgroundScroll_Pre_Y(uint8_t background, int16_t y);


WASM("tile", "get_background_scroll_pre")
extern position_s16_t tile_Get_BackgroundScroll_Pre(uint8_t background);

WASM("tile", "get_background_scroll_pre_x")
extern int16_t tile_Get_BackgroundScroll_Pre_X(uint8_t background);

WASM("tile", "get_background_scroll_pre_y")
extern int16_t tile_Get_BackgroundScroll_Pre_Y(uint8_t background);


WASM("tile", "set_background_scroll_post")
extern void tile_Set_BackgroundScroll_Post(uint8_t background, position_s16_t pos);

WASM("tile", "set_background_scroll_post_x")
extern void tile_Set_BackgroundScroll_Post_X(uint8_t background, int16_t x);

WASM("tile", "set_background_scroll_post_y")
extern void tile_Set_BackgroundScroll_Post_Y(uint8_t background, int16_t y);


WASM("tile", "get_background_scroll_post")
extern position_s16_t tile_Get_BackgroundScroll_Post(uint8_t background);

WASM("tile", "get_background_scroll_post_x")
extern int16_t tile_Get_BackgroundScroll_Post_X(uint8_t background);

WASM("tile", "get_background_scroll_post_y")
extern int16_t tile_Get_BackgroundScroll_Post_Y(uint8_t background);

WASM("tile", "set_background_transformation_matrix")
extern void tile_Set_BackgroundMatrix(uint8_t background, int16_t a, int16_t b, int16_t c, int16_t d);

#ifdef __cplusplus
}
#endif
