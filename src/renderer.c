#include <glad/glad.h>
#include <stdint.h>
#include <stdlib.h>
#include <stdio.h>
#include "renderer.h"
#include "engine.h"

// typedef struct {
//     RendererVerticesTypes type;
//     uint32_t data_capacity;
//     uint32_t data_size;
//     void *data;
// } VertexData;


// #define verts(x) ((VertexData *)(x))

// void *renderer_create_vertices(RendererVerticesTypes type) {
//     VertexData *vertex_data = malloc(1 * sizeof(*vertex_data));
//     if (!vertex_data) {
//         fprintf(stderr, "Error: allocation error\n");
//     }
//     vertex_data->type = type;
//     vertex_data->data_size = 0;
//     vertex_data->data_capacity = 10;
//     vertex_data->data = malloc(vertex_data->data_capacity);
//     return vertex_data;
// }

// void renderer_destroy_vertices(void *vertex_data) {
//     free(verts(vertex_data)->data);
//     free(vertex_data);
// }

#define to_mesh(x) ((Mesh *)(x))

typedef struct {
    uint32_t vao;
    uint32_t vbo;
    uint32_t prog;
    uint32_t vertex_count;
} Mesh;

void *renderer_mesh_create(const float *vertices, uint32_t vertex_count, uint32_t layout_location_count, const uint32_t vertex_layout[32], const char *vertex_shader_path, char *fragment_shader_path) {
    uint32_t vertices_size = sizeof(float);
    vertices_size *= engine_sum_u32(vertex_layout, layout_location_count);
    vertices_size *= vertex_count;

    Mesh *mesh = malloc(1 * sizeof *mesh);
    mesh->vertex_count = vertex_count;
    glGenBuffers(1, &mesh->vbo);
    glBindBuffer(GL_ARRAY_BUFFER, mesh->vbo);
    glBufferData(GL_ARRAY_BUFFER, vertices_size, vertices, GL_STATIC_DRAW);
    glGenVertexArrays(1, &mesh->vao);
    glBindVertexArray(mesh->vao);
    for (uint32_t i = 0; i < layout_location_count; i++) {
        glEnableVertexAttribArray(i);
        glVertexAttribPointer(i, vertex_layout[i], GL_FLOAT, GL_FALSE, vertices_size / vertex_count, (i == 0) ? (const void *)0 : (const void *)(uint64_t)(vertex_layout[i - 1] * sizeof(float)));
    }
    const char *vs_source = engine_read_file(vertex_shader_path);
    const char *fs_source = engine_read_file(fragment_shader_path);

    int success;
    char info_log[512];

    uint32_t vs = glCreateShader(GL_VERTEX_SHADER);
    glShaderSource(vs, 1, &vs_source, NULL);
    glCompileShader(vs);
    glGetShaderiv(vs, GL_COMPILE_STATUS, &success);
    if (!success) {
        glGetShaderInfoLog(vs, 512, NULL, info_log);
        printf("Vertex Shader Error: %s", info_log);
    }

    uint32_t fs = glCreateShader(GL_FRAGMENT_SHADER);
    glShaderSource(fs, 1, &fs_source, NULL);
    glCompileShader(fs);
    glGetShaderiv(fs, GL_COMPILE_STATUS, &success);
    if (!success) {
        glGetShaderInfoLog(fs, 512, NULL, info_log);
        printf("Fragment Shader Error: %s", info_log);
    }

    mesh->prog = glCreateProgram();
    glAttachShader(mesh->prog, vs);
    glAttachShader(mesh->prog, fs);
    glLinkProgram(mesh->prog);
    glGetProgramiv(mesh->prog, GL_LINK_STATUS, &success);
    if (!success) {
        glGetProgramInfoLog(mesh->prog, 512, NULL, info_log);
        printf("Shader Program Error: %s\n", info_log);
    }

    glDetachShader(mesh->prog, vs);
    glDetachShader(mesh->prog, fs);

    return (void *)mesh;
}

void renderer_mesh_destroy(void *mesh) {
    glDeleteVertexArrays(1, &to_mesh(mesh)->vao);
    glDeleteBuffers(1, &to_mesh(mesh)->vbo);
    glDeleteProgram(to_mesh(mesh)->prog);
    free(mesh);
}

void renderer_mesh_draw(void *mesh, float *model) {
    if (model != NULL) {
        renderer_update_model_matrix(mesh, model);
    }

    glBindVertexArray(to_mesh(mesh)->vao);
    glBindBuffer(GL_ARRAY_BUFFER, to_mesh(mesh)->vbo);
    glUseProgram(to_mesh(mesh)->prog);
    glDrawArrays(GL_TRIANGLES, 0, to_mesh(mesh)->vertex_count);
}
void renderer_update_model_matrix(void *mesh, float *matrix) {
    glUseProgram(to_mesh(mesh)->prog);
    glUniformMatrix4fv(glGetUniformLocation(to_mesh(mesh)->prog, "model"), 1, GL_FALSE, matrix);
}


uint32_t renderer_mesh_get_vao(void *mesh) {
    return to_mesh(mesh)->vao;
}

uint32_t renderer_mesh_get_vbo(void *mesh) {
    return to_mesh(mesh)->vbo;
}

uint32_t renderer_mesh_get_prog(void *mesh) {
    return to_mesh(mesh)->prog;
}
