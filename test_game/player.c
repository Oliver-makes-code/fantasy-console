#include "player.h"
#include <sprite.h>

player_t player = {
    .position = {
        .x = 30,
        .y = 31
    },
    
};

void update_player() {
    sprite_Set_Position(0, player.position);
}
