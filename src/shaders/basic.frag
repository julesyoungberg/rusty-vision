#version 450

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 frag_color;

layout(set = 0, binding = 0) uniform Uniforms {
    float time;
};

//@import util
//@import util2

void main() {
    vec3 color = vec3(abs(sin(time)));
    frag_color = vec4(color, 1);
}
