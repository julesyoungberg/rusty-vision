#version 450

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 frag_color;

layout(set = 0, binding = 0) uniform GeneralUniforms {
    vec2 mouse;
    vec2 resolution;
    float time;
};

void main() {
    vec2 st = uv * 0.5 + 0.5;
    st.y *= resolution.y / resolution.x;
    
    vec3 color = vec3(0);

    color += sin(st.x * 6.0 + sin(time + st.y * 90.0 + cos(st.x * 30.0 + time * 2.0))) * 0.5;
    color.rg += st;

    frag_color = vec4(color, 1);
}
