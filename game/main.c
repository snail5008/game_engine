#include <stdio.h>
#include <stdlib.h>
#include "../src/window.h"
#include "../src/engine.h"

void ready(void) {
    // yay I can read files. Such fun.
    const char *contents = engine_read_file("src/engine.c");
    printf("%s\n", contents);
    free((void *)contents);
}

void update(void) {
    printf("%f, %f\n", window_mouse_position_normalised().x, window_mouse_position_normalised().y);
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
