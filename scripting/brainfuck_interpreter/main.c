/*
    modified version of this interpreter designed to work with my game engine -- just a bit of fun!
*/

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <stdbool.h>

#define CELL_COUNT 1000000

const char* ReadFile(const char* filename, int max_file_size) {
	char* contents;
	contents = (char*)malloc(max_file_size);
	if (contents == NULL) {
		printf("{ERROR} - failed to allocate %d bytes", max_file_size);
		exit(1);
	}
	int ptr = 0;
	FILE* f = fopen(filename, "r");
	if (!f) {
		printf("{ERROR} - could not read file '%s'\n", filename);
		exit(1);
	}
	char c = fgetc(f);
	while (c != EOF) {
		if (ptr > max_file_size) {
			printf("{ERROR} - file too large\n");
			exit(1);
		}
		*(contents + ptr) = c;
		ptr++;
		c = fgetc(f);
	}
	*(contents + ptr) = '\0';
	fclose(f);
	return contents;
}

int main(int argc, char** argv) {
    printf("A random brainfuck interpreter v0.1.0\n");

    const char* code;
    bool debug_mode = false;

    if (argc > 1) {
        code = ReadFile(argv[1], CELL_COUNT);
        for (int i = 0; i < argc; i++) {
            if (strcmp(argv[i], "debug") == 0) {
                debug_mode = true;
                printf("Running in debug mode\n");
            }
        }
    }
    else {
        printf("ERROR: no file specified\n");
        exit(1);
    }
    
    char* pointer;

    /* initialize cells */
    pointer = malloc(CELL_COUNT);
    for (int i = 0; i < CELL_COUNT; i++) {
        pointer[i] = 0;
    }

    uint32_t i = 0;

    /* loop throught `code` */
    while (i < strlen(code)) {

        if (code[i] == '+') {
            (*pointer)++;
            i++;
        }
        else if (code[i] == '-') {
            (*pointer)--;
            i++;
        }

        else if (code[i] == '>') {
            pointer++;
            i++;
        }
        else if (code[i] == '<') {
            pointer--;
            i++;
        }

        else if (code[i] == '.') {
            if (debug_mode)
                printf("%d : %c | ", *pointer, *pointer);
            else
                printf("%c", *pointer);
            i++;
        }
        else if (code[i] == ',') {
            *pointer = getchar();
            i++;
        }

        else if (code[i] == '[') {
            if (!*pointer) {
                while (code[i] != ']') {
                    i++;
                }
            }
            else {
                i++;
            }
        }

        else if (code[i] == ']') {
            if (*pointer) {
                while (code[i] != '[') {
                    i--;
                }
            }
            else {
                i++;
            }
        }
        else {
            i++;
        }
    }

    if (debug_mode) {
        printf("\n\nFIRST 10 CELLS:\n\n");
        for (int i = 0; i < 10; i++) {
            printf("%d:%c | ", pointer[i], pointer[i]);
        }
        puts("");
    }

    return 0;
}
