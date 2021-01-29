#version 450

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 frag_color;

/**
 * The main uniform buffer. This needs to be included in every shader.
 */
layout(set = 0, binding = 0) uniform GeneralUniforms {
    int colorMode;
    float time;
    vec2 resolution;
    float color1R;
    float color1G;
    float color1B;
    float color2R;
    float color2G;
    float color2B;
    float color3R;
    float color3G;
    float color3B;
};

void main() {
    frag_color = vec4(uv, abs(sin(time)), 1);
}
