#pragma once

#include <common_types.h>

typedef struct {
    position_s16_t position;
    position_s16_t velocity;
} player_t;

void update_player();
