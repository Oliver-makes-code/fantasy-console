#pragma once
#include <stdint.h>
#define WASM(mod, name) __attribute__((import_module(mod), import_name(name)))

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    int16_t x;
    int16_t y;
} backgroundscroll_t;

WASM("tile", "write_palette")
extern void tile_WritePalette(uint8_t palette, uint32_t color);

WASM("tile", "write_tile")
extern void tile_WriteTile(uint8_t palette, const uint8_t *tile);

WASM("tile", "set_background_tile")
extern void tile_SetBackgroundTile(uint8_t background, uint8_t x, uint8_t y, uint8_t tile);



WASM("tile", "set_background_scroll_pre")
extern void tile_Set_BackgroundScroll_Pre(uint8_t background, int16_t x, int16_t y);

WASM("tile", "set_background_scroll_pre_x")
extern void tile_Set_BackgroundScroll_Pre_X(uint8_t background, int16_t x);

WASM("tile", "set_background_scroll_pre_y")
extern void tile_Set_BackgroundScroll_Pre_Y(uint8_t background, int16_t y);

WASM("tile", "set_background_scroll_post")
extern void tile_Set_BackgroundScroll_Post(uint8_t background, int16_t x, int16_t y);

WASM("tile", "set_background_scroll_post_x")
extern void tile_Set_BackgroundScroll_Post_X(uint8_t background, int16_t x);

WASM("tile", "set_background_scroll_post_y")
extern void tile_Set_BackgroundScroll_Post_Y(uint8_t background, int16_t y);

WASM("tile", "get_background_scroll_pre")
extern backgroundscroll_t tile_Get_BackgroundScroll_Pre(uint8_t background);

WASM("tile", "get_background_scroll_pre_x")
extern int16_t tile_Get_BackgroundScroll_Pre_X(uint8_t background);

WASM("tile", "get_background_scroll_pre_y")
extern int16_t tile_Get_BackgroundScroll_Pre_Y(uint8_t background);

WASM("tile", "get_background_scroll_post")
extern backgroundscroll_t tile_Get_BackgroundScroll_Post(uint8_t background);

WASM("tile", "get_background_scroll_post_x")
extern int16_t tile_Get_BackgroundScroll_Post_X(uint8_t background);

WASM("tile", "get_background_scroll_post_y")
extern int16_t tile_Get_BackgroundScroll_Post_Y(uint8_t background);

WASM("tile", "set_background_transformation_matrix")
extern void tile_Set_BackgroundMatrix(uint8_t background, int16_t a, int16_t b, int16_t c, int16_t d);

#ifdef  __cplusplus
}
#endif