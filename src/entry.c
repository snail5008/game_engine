#include <stdio.h>
#include "window.h"

int game_main(void);

int main(void) {
    window_create(800, 600, "Game");
    game_main();
    window_destroy();
    return 0;
}
