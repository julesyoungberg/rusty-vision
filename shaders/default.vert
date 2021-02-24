#version 450

layout(location = 0) in vec2 position;
layout(location = 0) out vec2 uv;
layout(location = 1) out vec2 st;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    uv = position.xy;
    st = uv * 0.5 + 0.5;
}
