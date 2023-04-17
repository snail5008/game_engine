#version 330 core
layout (location = 0) in vec2 aPos;
layout (location = 1) in vec3 aColour;
out vec3 vertex_colour;

void main() {
    gl_Position = vec4(aPos, 0.0, 1.0);
    vertex_colour = aColour;
}
