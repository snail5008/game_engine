#include <stdint.h>
#include <stdbool.h>

void window_create(uint32_t width, uint32_t height, const char *title);
void window_destroy(void);
bool window_open(void);
void window_frame_begin(void);
void window_frame_end(void);
void window_background_colour(float r, float g, float b);
