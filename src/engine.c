#include "engine.h"
#include <stdlib.h>
#include <stdint.h>
#include <stdio.h>

void engine_capacityfunc_add(uint32_t *capacity) {
    *capacity = *capacity + ENGINE_CAPACITY_ADD_BLOCK_SIZE;
}

void engine_capacityfunc_double(uint32_t *capacity) {
    *capacity = *capacity * 2;
}

char *engine_read_file_custom_capacity_increase(const char *filename, custom_fileread_capacity_t capacityfunc) {
    FILE *file = fopen(filename, "r");
    if (!file) {
        fprintf(stderr, "Error: could not read file\n");
        exit(1);
    }

    uint32_t capacity = 100;
    uint32_t length = 0;
    char c;
    char *contents = malloc(capacity * sizeof *contents);
    if (!contents) {
        fprintf(stderr, "Error: allocation error\n");
        exit(1);
    }

    while ((c = getc(file)) != EOF) {
        if (length >= capacity - 1) {
            capacityfunc(&capacity);
            contents = realloc(contents, capacity * sizeof *contents);
            if (!contents) {
                fprintf(stderr, "Error: allocation error\n");
                exit(1);
            }
        }
        contents[length++] = c;
    }
    contents[length] = '\0';

    fclose(file);
    
    return contents;
}

char *engine_read_file(const char *filename) {
    return engine_read_file_custom_capacity_increase(filename, engine_capacityfunc_add);
}

uint32_t engine_sum_u32(uint32_t *vector, uint32_t vector_element_count) {
    uint32_t result = 0;
    for (uint32_t i = 0; i < vector_element_count; i++) {
        result += vector[i];
    }
    return result;
}
