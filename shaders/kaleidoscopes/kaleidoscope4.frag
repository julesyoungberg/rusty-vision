#version 450

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 frag_color;

layout(set = 0, binding = 0) uniform GeneralUniforms {
    vec2 mouse;
    vec2 resolution;
    float time;
    int mouse_down;
};

layout(set = 1, binding = 0) uniform sampler spectrum_sampler;
layout(set = 1, binding = 1) uniform texture2D spectrum;

vec2 map_coord(in vec2 uv) {
    return uv;
}

void main() {
    vec2 st = uv;
    st.x *= resolution.x / resolution.y;

    vec3 color = vec3(0.0);

    st = map_coord(uv);
    color.rg = st;

    frag_color = vec4(color, 1);
}
