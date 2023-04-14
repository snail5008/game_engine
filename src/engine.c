#include "engine.h"
#include <stdlib.h>
#include <stdint.h>
#include <stdio.h>

char *engine_read_file(const char *filename) {
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
            capacity += 100;
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
