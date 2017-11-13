#version 150 core

in vec2 xy;

void main() {
    gl_Position = vec4(xy, 0.0, 1);
}
