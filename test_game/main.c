#include "player.h"
#include <common_types.h>
#include <stdint.h>
#include <debug.h>
#include <tile.h>
#include <sprite.h>

const uint32_t PALETTE_GROUND[] = {
    0x796543,
    0x816f48,
    0x897a4e,
    0x908353,
    0x998d58,
    0x008c32,
    0x159831,
    0x29a430,
    0x3eb02f,
    0x52bc2e,
    0x8080FF,
};

const uint8_t TILE_GRASS[] = {
    0x89, 0xA8, 0x9A, 0x8A, 0x98, 0xA9, 0x8A, 0xA9, 
    0x7A, 0x87, 0x89, 0x7A, 0x87, 0x98, 0xA8, 0x98, 
    0x78, 0x77, 0xA8, 0x78, 0x97, 0x89, 0x79, 0x87, 
    0x86, 0x79, 0x87, 0x98, 0x77, 0x87, 0x78, 0x67, 
    0x72, 0x78, 0x77, 0x86, 0x17, 0x62, 0x76, 0x27, 
    0x61, 0x27, 0x61, 0x72, 0x26, 0x12, 0x26, 0x12, 
    0x13, 0x16, 0x22, 0x61, 0x21, 0x23, 0x36, 0x12, 
    0x24, 0x32, 0x43, 0x12, 0x33, 0x44, 0x31, 0x21, 
    0x12, 0x23, 0x32, 0x14, 0x34, 0x53, 0x22, 0x13, 
    0x43, 0x21, 0x21, 0x23, 0x55, 0x44, 0x21, 0x33, 
    0x53, 0x42, 0x12, 0x45, 0x43, 0x32, 0x11, 0x44, 
    0x44, 0x33, 0x21, 0x34, 0x32, 0x12, 0x12, 0x35, 
    0x55, 0x32, 0x12, 0x23, 0x12, 0x42, 0x21, 0x43, 
    0x43, 0x42, 0x12, 0x12, 0x24, 0x33, 0x41, 0x23, 
    0x34, 0x31, 0x11, 0x21, 0x33, 0x44, 0x32, 0x13, 
    0x33, 0x21, 0x22, 0x12, 0x34, 0x55, 0x44, 0x12
};

const uint8_t TILE_DIRT[] = {
    0x21, 0x12, 0x24, 0x21, 0x25, 0x45, 0x43, 0x41, 
    0x11, 0x24, 0x33, 0x41, 0x13, 0x54, 0x54, 0x32, 
    0x23, 0x43, 0x44, 0x32, 0x13, 0x45, 0x43, 0x21, 
    0x34, 0x34, 0x54, 0x34, 0x21, 0x34, 0x32, 0x12, 
    0x44, 0x54, 0x54, 0x43, 0x12, 0x23, 0x41, 0x23, 
    0x34, 0x45, 0x43, 0x32, 0x11, 0x22, 0x12, 0x12, 
    0x33, 0x44, 0x34, 0x31, 0x24, 0x33, 0x21, 0x12, 
    0x24, 0x34, 0x43, 0x22, 0x33, 0x44, 0x31, 0x21, 
    0x12, 0x23, 0x32, 0x14, 0x34, 0x53, 0x22, 0x13, 
    0x43, 0x21, 0x21, 0x23, 0x55, 0x44, 0x21, 0x33, 
    0x53, 0x42, 0x12, 0x45, 0x43, 0x32, 0x11, 0x44, 
    0x44, 0x33, 0x21, 0x34, 0x32, 0x12, 0x12, 0x35, 
    0x55, 0x32, 0x12, 0x23, 0x12, 0x42, 0x21, 0x43, 
    0x43, 0x42, 0x12, 0x12, 0x24, 0x33, 0x41, 0x23, 
    0x34, 0x31, 0x11, 0x21, 0x33, 0x44, 0x32, 0x13, 
    0x33, 0x21, 0x22, 0x12, 0x34, 0x55, 0x44, 0x12
};

void init() {
    dbg_WriteString("Hello, world!\n");
    for (uint8_t i = 0; i <= 10; i++)
        tile_WritePalette(i, PALETTE_GROUND[i]);
    tile_WriteTile(1, TILE_GRASS);
    tile_WriteTile(2, TILE_DIRT);

    tile_Set_BackgroundPalette(10);

    for (int16_t i = 0; i < 20; i++) {
        tile_Set_BackgroundTile(0, (position_u8_t) {i, 13}, 1);
        tile_Set_BackgroundTile(0, (position_u8_t) {i, 14}, 2);
    }

    sprite_Set_Visible(0, true);
    sprite_Set_Tile(0, 2);
}

void update() {
    update_player();
}

void v_blank(uint8_t y) {

}
