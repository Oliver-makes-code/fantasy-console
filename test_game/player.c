#include "player.h"
#include <sprite.h>

player_t player = {
    .position = {
        .x = 30,
        .y = 31
    },
    .velocity = {
        .x = 5,
        .y = 1
    }
};

void update_player() {
    player.position.x += player.velocity.x;
    player.position.y += player.velocity.y;
    player.velocity.y++;
    player.position.x %= 320;
    player.position.x += 320;
    player.position.x %= 320;
    player.position.y %= 240;
    player.position.y += 240;
    player.position.y %= 240;

    sprite_Set_Position(0, player.position);
}
