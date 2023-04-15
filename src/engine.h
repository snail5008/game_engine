#include <stdint.h>

#if !defined(ENGINE_CAPACITY_ADD_BLOCK_SIZE)
    #define ENGINE_CAPACITY_ADD_BLOCK_SIZE 256
#endif

typedef void (*custom_fileread_capacity_t)(uint32_t *capacity);

void engine_capacityfunc_add(uint32_t *capacity);
void engine_capacityfunc_double(uint32_t *capacity);
char *engine_read_file_custom_capacity_increase(const char *filename, custom_fileread_capacity_t capacityfunc);
char *engine_read_file(const char *filename);
