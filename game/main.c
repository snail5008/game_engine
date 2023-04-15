#include <stdio.h>
#include <stdlib.h>
#include "../src/window.h"
#include "../src/engine.h"

void ready(void) {
    printf("Window width: %d\n", window_width());
    printf("Window height: %d\n", window_height());
    printf("Window title: %s\n", window_title());
}

void update(void) {
    // printf("%f, %f\n", window_mouse_accel().x, window_mouse_accel().y);
}

void terminate(void) {

}

int game_main(void) {
    ready();
    while (window_open()) {
        window_frame_begin();
        update();
        window_frame_end();
    }
    terminate();
    return 0;
}
