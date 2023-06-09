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
    Vec2i32 mouse_position;
    Vec2f32 mouse_accel;
    mouse_buttons_t mouse_buttons;
    float mouse_wheel;
    double frame_start_time;
    double frame_time;
} Window;

static Window window = {
    .initialised = false,
    .bg_colour = {
        .r = 0.2,
        .g = 0.2,
        .b = 0.2,
    }
};

void glfw_mousepos_callback(GLFWwindow *glfw_window, double x, double y) {
    (void)glfw_window;
    window.mouse_accel.x = x / window.width - window_mouse_position_normalised().x;
    window.mouse_accel.y = y / window.height - window_mouse_position_normalised().y;
    window.mouse_position.x = x;
    window.mouse_position.y = y;
}

void glfw_windowsize_callback(GLFWwindow *glfw_window, int width, int height) {
    window.width = width;
    window.height = height;
    (void)glfw_window;
    glViewport(0, 0, window.width, window.height);
}

void glfw_mousebutton_callback(GLFWwindow *glfw_window, int button, int action, int mods) {
    (void)glfw_window;
    (void)mods;
    if (button == GLFW_MOUSE_BUTTON_LEFT) {
        if (action == GLFW_PRESS) {
            window.mouse_buttons |= MOUSE_BUTTON_LEFT;
        } else if (action == GLFW_RELEASE) {
            window.mouse_buttons &= ~MOUSE_BUTTON_LEFT;
        }
    } else if (button == GLFW_MOUSE_BUTTON_RIGHT) {
        if (action == GLFW_PRESS) {
            window.mouse_buttons |= MOUSE_BUTTON_RIGHT;
        } else if (action == GLFW_RELEASE) {
            window.mouse_buttons &= ~MOUSE_BUTTON_RIGHT;
        }
    } else if (button == GLFW_MOUSE_BUTTON_MIDDLE) {
        if (action == GLFW_PRESS) {
            window.mouse_buttons |= MOUSE_BUTTON_MIDDLE;
        } else if (action == GLFW_RELEASE) {
            window.mouse_buttons &= ~MOUSE_BUTTON_MIDDLE;
        }
    }
}

void glfw_scrollwheel_callback(GLFWwindow *glfw_window, double x_offset, double y_offset) {
    (void)glfw_window;
    (void)x_offset;
    window.mouse_wheel = y_offset;
}

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
        //4.5
    }

    glfwSwapInterval(1);
    glfwSetCursorPosCallback(window.window, glfw_mousepos_callback);
    glfwSetMouseButtonCallback(window.window, glfw_mousebutton_callback);
    glfwSetScrollCallback(window.window, glfw_scrollwheel_callback);
    glfwSetWindowSizeCallback(window.window, glfw_windowsize_callback);

    glEnable(GL_DEPTH_TEST);

    window.mouse_position.x = 0;
    window.mouse_position.y = 0;
    window.mouse_wheel = 0;

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
    window.frame_start_time = glfwGetTime();

    glClearColor(window.bg_colour.r, window.bg_colour.g, window.bg_colour.b, 0.0);
    glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
}

void window_frame_end(void) {
    window.frame_time = glfwGetTime() - window.frame_start_time;

    window.mouse_wheel = 0;
    window.mouse_accel.x = 0;
    window.mouse_accel.y = 0;
    glfwSwapBuffers(window.window);
    glfwPollEvents();
}

void window_background_colour(float r, float g, float b) {
    window.bg_colour.r = r;
    window.bg_colour.g = g;
    window.bg_colour.b = b;
}

Vec2i32 window_mouse_position(void) {
    return window.mouse_position;
}

Vec2f32 window_mouse_position_normalised(void) {
    Vec2f32 vec;
    vec.x = window_mouse_position().x / (float)window.width;
    vec.y = window_mouse_position().y / (float)window.height;
    return vec;
}

Vec2f32 window_mouse_accel(void) {
    return window.mouse_accel;
}

uint32_t window_width(void) {
    return window.width;
}

uint32_t window_height(void) {
    return window.height;
}

const char *window_title(void) {
    return window.title;
}

void window_set_width(uint32_t width) {
    window.width = width;
    glfwSetWindowSize(window.window, window.width, window.height);
}

void window_set_height(uint32_t height) {
    window.height = height;
    glfwSetWindowSize(window.window, window.width, window.height);
}

void window_set_title(const char *title) {
    window.title = title;
    glfwSetWindowTitle(window.window, window.title);
}

mouse_buttons_t window_mouse_buttons(void) {
    return window.mouse_buttons;
}

float window_scroll_wheel_direction(void) {
    return window.mouse_wheel;
}

double window_frametime(void) {
    return window.frame_time;
}
