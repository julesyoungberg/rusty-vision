#version 450

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 frag_color;

layout(set = 0, binding = 0) uniform Uniforms {
    float time;
} uniforms;

void main() {
    frag_color = vec4(uniforms.time / 1000.0, uv, 1);
}
