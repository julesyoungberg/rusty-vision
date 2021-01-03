#version 450

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 frag_color;

layout(set = 0, binding = 0) uniform Uniforms {
    int colorMode;
    bool drawFloor;
    float fogDist;
    float quality;
    vec2 resolution;
    float shapeColorR;
    float shapeColorG;
    float shapeColorB;
    float time;
    float paletteColor1R;
    float paletteColor1G;
    float paletteColor1B;
    float paletteColor2R;
    float paletteColor2G;
    float paletteColor2B;
    float paletteColor3R;
    float paletteColor3G;
    float paletteColor3B;
};

//@import util
//@import util2

void main() {
    frag_color = vec4(uv, abs(sin(time)), 1);
}
