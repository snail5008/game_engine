#pragma once
#include <stdint.h>
#include <stdbool.h>
#include "vector.h"

typedef uint32_t mouse_buttons_t;
#define MOUSE_BUTTON_LEFT 1
#define MOUSE_BUTTON_RIGHT 2
#define MOUSE_BUTTON_MIDDLE 4

void window_create(uint32_t width, uint32_t height, const char *title);
void window_destroy(void);
bool window_open(void);
void window_frame_begin(void);
void window_frame_end(void);
void window_background_colour(float r, float g, float b);
Vec2i32 window_mouse_position(void);
Vec2f32 window_mouse_position_normalised(void);
Vec2f32 window_mouse_accel(void);
uint32_t window_width(void);
uint32_t window_height(void);
const char *window_title(void);
void window_set_width(uint32_t width);
void window_set_height(uint32_t height);
void window_set_title(const char *title);
mouse_buttons_t window_mouse_buttons(void);
float window_scroll_wheel_direction(void);
double window_frametime(void);
