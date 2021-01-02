#version 450

layout(location = 0) in vec2 position;
layout(location = 0) out vec2 uv;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    uv = vec2(position.x * 0.5 + 0.5, position.y * 0.5 + 0.5);
}
