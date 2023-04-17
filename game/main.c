#include <stdio.h>
#include <stdlib.h>
#include "../src/window.h"
#include "../src/engine.h"
#include "../src/renderer.h"

float triangle[] = {
 /*     pos          colours     opacity    */
     0.0,  1.0,   1.0, 0.0, 0.0,   0.5,
     1.0, -1.0,   0.0, 1.0, 0.0,   1.0,
    -1.0, -1.0,   0.0, 0.0, 1.0,   1.0
};

void *mesh;

void ready(void) {
    printf("Window width: %d\n", window_width());
    printf("Window height: %d\n", window_height());
    printf("Window title: %s\n", window_title());

    mesh = renderer_mesh_create(triangle, 3, 3, (uint32_t[]) {2, 3, 1}, "shaders/default.vert", "shaders/default.frag");

}

void update(void) {
    // printf("%f\n", window_scroll_wheel_direction());
    renderer_mesh_draw(mesh);
}

void terminate(void) {
    renderer_mesh_destroy(mesh);
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
