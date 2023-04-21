#pragma once

#include <stdint.h>

void bf_init(void *s, const char *program_file_name);
void bf_set_cell(void *s, uint8_t c);
uint8_t bf_get_cell(void *s);
void bf_next_cell(void *s);
void bf_prev_cell(void *s);
void bf_inc_cell(void *s);
void bf_dec_cell(void *s);
uint8_t bf_next_prog_char(void *s);
uint8_t bf_prog_char(void *s);
void bf_execute_char(void *s);
