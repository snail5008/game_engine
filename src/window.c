#include <glad/glad.h>
#include <GLFW/glfw3.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

#include "window.h"

typedef struct {
    GLFWwindow *window;
    uint32_t width;
    uint32_t height;
    const char *title;
    bool initialised;
    struct {
        float r;
        float g;
        float b;
    } bg_colour;
} Window;

static Window window = {
    .initialised = false,
    .bg_colour = {
        .r = 0.2,
        .g = 0.2,
        .b = 0.2,
    }
};

void window_create(uint32_t width, uint32_t height, const char *title) {
    if (window.initialised) {
        fprintf(stderr, "Error: attempted window initialisation multiple times\n");
        exit(1);
    }

    if (!glfwInit()) {
        fprintf(stderr, "Error: could not initialise GLFW\n");
        exit(1);
    }

    glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
    glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 3);
    glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);

    window.width = width;
    window.height = height;
    window.title = title;
    window.window = glfwCreateWindow(width, height, title, NULL, NULL);

    if (!window.window) {
        fprintf(stderr, "Error: could not create window\n");
        exit(1);
    }

    glfwMakeContextCurrent(window.window);
    
    if (!gladLoadGL()) {
        fprintf(stderr, "Error: could not load OpenGL\n");
        exit(1);
    }

    glfwSwapInterval(1);

    window.initialised = true;
}

void window_destroy(void) {
    if (!window.initialised) {
        fprintf(stderr, "Error: attempted to destroy uninitialised window\n");
        exit(1);
    }

    glfwDestroyWindow(window.window);
    glfwTerminate();

    window.initialised = false;
}

bool window_open(void) {
    return !glfwWindowShouldClose(window.window);
}

void window_frame_begin(void) {
    glClearColor(window.bg_colour.r, window.bg_colour.g, window.bg_colour.b, 0.0);
    glClear(GL_COLOR_BUFFER_BIT);
}

void window_frame_end(void) {
    glfwSwapBuffers(window.window);
    glfwPollEvents();
}

void window_background_colour(float r, float g, float b) {
    window.bg_colour.r = r;
    window.bg_colour.g = g;
    window.bg_colour.b = b;
}
