#pragma once
#include <stdint.h>

typedef union {
    struct {
        int32_t x;
        int32_t y;
    };
    struct {
        int32_t u;
        int32_t v;
    };
    struct {
        int32_t s;
        int32_t t;
    };
    int32_t data[2];
} Vec2i32;

typedef union {
    struct {
        float x;
        float y;
    };
    struct {
        float u;
        float v;
    };
    struct {
        float s;
        float t;
    };
    float data[2];
} Vec2f32;
