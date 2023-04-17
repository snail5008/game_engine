#version 330 core
out vec4 frag_colour;
in vec3 vertex_colour;

void main() {
    frag_colour = vec4(vertex_colour, 1.0);
}
