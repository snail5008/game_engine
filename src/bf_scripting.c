/*
    modified version of this interpreter designed to work with my game engine -- just a bit of fun!

    (required) modifications to integrate brainfuck with existing programs:
        - an ability to call functions with arguments. To keep in the spirit of brainfuck being a ridiculous language
          to program in, the programmer will need to push each letter of the function name, and after a NUL, the arguments
          with no checks for bounds! Because that will be fun :) Actually, this will not be implemented by this module itself;
          rather, it will provide functions to: (A) check what the next character is, and (B) to execute the character normally
          or do something else. If the caller module decides to do something else, it will have full access over all runtime data
          (which is now just the cell and the program buffers [they are kept separate]). This means that for the function
          example, the user could:
            read a char, see that it is % (percent sign)
            continue forwards until a %
            have some data after that that will be interpreted by the caller module:
                for example, if the function is 'window_create(int, int, const char * %terminated), then the cell data from thefirst %
                could look something like this (numbers will be hex and take up 4 bytes each, because int):
                    %window_create%800600Game%
*/

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <stdbool.h>
#include "engine.h"

#define to_state(x) ((State *)(x))

typedef struct {
    size_t cell_index;
    size_t program_index;
    uint8_t *cell_data;
    uint8_t *program_data;
} State;


void *bf_init(const char *program_file_name) {
    State *s = malloc(sizeof *s);
    s->cell_data = malloc(1024);
    s->program_data = (uint8_t *)engine_read_file(program_file_name);
    return (void *)s;
}

void bf_terminate(void *s) {
    free(to_state(s)->cell_data);
    free(to_state(s)->program_data);
    free(s);
}

void bf_set_cell(void *s, uint8_t c) {
    to_state(s)->cell_data[to_state(s)->cell_index] = c;
}

uint8_t bf_get_cell(void *s) {
    return to_state(s)->cell_data[to_state(s)->cell_index];
}

// in future, reallocate if needed
void bf_next_cell(void *s) {
    to_state(s)->cell_index++;
}

void bf_prev_cell(void *s) {
    to_state(s)->cell_index--;
}

void bf_inc_cell(void *s) {
    to_state(s)->cell_data[to_state(s)->cell_index]++;
}

void bf_dec_cell(void *s) {
    to_state(s)->cell_data[to_state(s)->cell_index]--;
}

uint8_t bf_next_prog_char(void *s) {
    return to_state(s)->program_data[to_state(s)->program_index++];
}

uint8_t bf_prog_char(void *s) {
    return to_state(s)->program_data[to_state(s)->program_index];
}

void bf_execute_char(void *s) {
    switch (bf_next_prog_char(s)) {
    case '>':
        bf_next_cell(s);
        break;
    case '<':
        bf_prev_cell(s);
        break;
    case '+':
        bf_inc_cell(s);
        break;
    case '-':
        bf_dec_cell(s);
        break;
    case '.':
        printf("%d", bf_get_cell(s));
        break;
    }
}


// OLD CODE -- this is actually complete shit, imma fix it above
// #define CELL_COUNT 1000000

// const char* ReadFile(const char* filename, int max_file_size) {
// 	char* contents;
// 	contents = (char*)malloc(max_file_size);
// 	if (contents == NULL) {
// 		printf("{ERROR} - failed to allocate %d bytes", max_file_size);
// 		exit(1);
// 	}
// 	int ptr = 0;
// 	FILE* f = fopen(filename, "r");
// 	if (!f) {
// 		printf("{ERROR} - could not read file '%s'\n", filename);
// 		exit(1);
// 	}
// 	char c = fgetc(f);
// 	while (c != EOF) {
// 		if (ptr > max_file_size) {
// 			printf("{ERROR} - file too large\n");
// 			exit(1);
// 		}
// 		*(contents + ptr) = c;
// 		ptr++;
// 		c = fgetc(f);
// 	}
// 	*(contents + ptr) = '\0';
// 	fclose(f);
// 	return contents;
// }

// int main(int argc, char** argv) {
//     printf("A random brainfuck interpreter v0.1.0\n");

//     const char* code;
//     bool debug_mode = false;

//     if (argc > 1) {
//         code = ReadFile(argv[1], CELL_COUNT);
//         for (int i = 0; i < argc; i++) {
//             if (strcmp(argv[i], "debug") == 0) {
//                 debug_mode = true;
//                 printf("Running in debug mode\n");
//             }
//         }
//     }
//     else {
//         printf("ERROR: no file specified\n");
//         exit(1);
//     }
    
//     char* pointer;

//     /* initialize cells */
//     pointer = malloc(CELL_COUNT); // che cazzo è questo???? Idk what I was thinking when I wrote this (1MB of space that's never going to be freed)
//     for (int i = 0; i < CELL_COUNT; i++) {
//         pointer[i] = 0;
//     }

//     uint32_t i = 0;

//     /* loop throught `code` */
//     while (i < strlen(code)) {

//         if (code[i] == '+') {
//             (*pointer)++;
//             i++;
//         }
//         else if (code[i] == '-') {
//             (*pointer)--;
//             i++;
//         }

//         else if (code[i] == '>') {
//             pointer++;
//             i++;
//         }
//         else if (code[i] == '<') {
//             pointer--;
//             i++;
//         }

//         else if (code[i] == '.') {
//             if (debug_mode)
//                 printf("%d : %c | ", *pointer, *pointer);
//             else
//                 printf("%c", *pointer);
//             i++;
//         }
//         else if (code[i] == ',') {
//             *pointer = getchar();
//             i++;
//         }

//         else if (code[i] == '[') {
//             if (!*pointer) {
//                 while (code[i] != ']') {
//                     i++;
//                 }
//             }
//             else {
//                 i++;
//             }
//         }

//         else if (code[i] == ']') {
//             if (*pointer) {
//                 while (code[i] != '[') {
//                     i--;
//                 }
//             }
//             else {
//                 i++;
//             }
//         }
//         else {
//             i++;
//         }
//     }

//     if (debug_mode) {
//         printf("\n\nFIRST 10 CELLS:\n\n");
//         for (int i = 0; i < 10; i++) {
//             printf("%d:%c | ", pointer[i], pointer[i]);
//         }
//         puts("");
//     }

//     return 0;
// }
