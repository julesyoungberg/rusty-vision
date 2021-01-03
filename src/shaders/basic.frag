#version 450

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 frag_color;

layout(set = 0, binding = 0) uniform Uniforms {
    int colorMode;
    bool drawFloor;
    float fogDist;
    int quality;
    vec2 resolution;
    bool spin;
    float time;
};

//@import util
//@import util2

void main() {
    frag_color = vec4(uv, abs(sin(time)), 1);
}
