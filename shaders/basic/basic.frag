#version 450

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 frag_color;

layout(set = 0, binding = 0) uniform GeneralUniforms {
    vec2 mouse;
    vec2 resolution;
    float time;
    int mouse_down;
};

void main() {
    vec2 st = uv;
    st.y *= resolution.y / resolution.x;
    vec3 color = vec3(st * 0.5 + 0.5, abs(sin(time)));
    float d = distance(st, 2.0 * mouse / resolution.x);
    color += smoothstep(0.01, 0.0, d);
    frag_color = vec4(color, 1);
}
