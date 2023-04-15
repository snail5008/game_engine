#pragma once
#include <stdint.h>
#include <stdbool.h>
#include "vector.h"

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
