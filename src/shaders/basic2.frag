#version 450

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 frag_color;

layout(set = 0, binding = 0) uniform Uniforms {
    int colorMode;
    bool drawFloor;
    float fogDist;
    float quality;
    vec2 resolution;
    float time;
    vec3 shapeColor;
};

void main() {
    frag_color = vec4(abs(sin(time)), uv, 1);
}
