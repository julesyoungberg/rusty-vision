#version 450

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 frag_color;

layout(set = 0, binding = 0) uniform Uniforms {
    float time;
};

//@import util
//@import util2

void main() {
    frag_color = vec4(time, time, time, 1);
}
