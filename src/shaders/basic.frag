#version 450

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 frag_color;

/**
 * The main uniform buffer. This needs to be included in every shader.
 */
layout(set = 0, binding = 0) uniform GeneralUniforms {
    float time;
    vec2 resolution;
};

void main() {
    frag_color = vec4(uv, abs(sin(time)), 1);
}
