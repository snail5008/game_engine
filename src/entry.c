#include <stdio.h>
#include <glad/glad.h>
#include "window.h"

int game_main(void);

int main(void) {
    // window_create(800, 600, "Game");
    game_main();
    printf("Application exited with OpenGL error 0x%x\n", glGetError());
    // window_destroy();
    return 0;
}
