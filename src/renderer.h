#pragma once
#include <stdint.h>

// typedef enum {RENDERER_VERTEXTYPE_2D, RENDERER_VERTEXTYPE_3D} RendererVerticesTypes;

// void *renderer_create_vertices(RendererVerticesTypes type);
// void renderer_delete_vertices(void *vertex_data);
// void renderer_push_vertex(void *vertex_data, void *vertex);

void *renderer_mesh_create(const float *vertices, uint32_t vertex_count, uint32_t layout_location_count, const uint32_t vertex_layout[32], const char *vertex_shader_path, char *fragment_shader_path);
void renderer_mesh_destroy(void *mesh);
void renderer_mesh_draw(void *mesh, float *model);
void renderer_update_model_matrix(void *mesh, float *matrix);

// future, allow delete opengl obj, replace with new one
uint32_t renderer_mesh_get_vao(void *mesh);
uint32_t renderer_mesh_get_vbo(void *mesh);
uint32_t renderer_mesh_get_prog(void *mesh);
