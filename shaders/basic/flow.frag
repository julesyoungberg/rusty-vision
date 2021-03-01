#version 450

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 frag_color;

layout(set = 0, binding = 0) uniform GeneralUniforms {
    vec2 mouse;
    vec2 resolution;
    float time;
};

#define PI 3.14159265359

// adapted from: https://editor.isf.video/shaders/5e7a80437c113618206dee05
void main() {
    vec2 st = uv * 0.5 + 0.5;;
    st.y *= resolution.y / resolution.x;
    st *= 2.0;

    vec3 color = vec3(0.0);

    float t1 = time * 1.13;
    float t2 = time * 0.9;

    for (float i = 1.0; i < 5.0; i += 1.0) {
        vec2 p = st;
        p.x += 0.75 / i * sin(i * PI * st.y + t1 * 0.1);
        p.y += 0.35 / i * cos(i * PI * st.x + t2);
        st = p;
    }

    float v = st.x + st.y;
    color = vec3(
        cos(v + 8.69) * 0.5 + 0.5,
        sin(v + 2.33) * 0.5 + 0.5,
        (sin(v + 13.0) + cos(v + 4.64)) * 0.25 + 0.5
    );

    frag_color = vec4(color * color, 1);
}
