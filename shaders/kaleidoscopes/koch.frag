#version 450

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 frag_color;

layout(set = 0, binding = 0) uniform GeneralUniforms {
    vec2 mouse;
    vec2 resolution;
    float time;
    int mouse_down;
};

#define PI 3.14159265359

vec2 N(float angle) {
    return vec2(sin(angle), cos(angle));
}

void main() {
    vec2 st = uv;
    st.x *= resolution.x / resolution.y;
    
    vec3 color = vec3(0.0);

    st.x = abs(st.x);

    float angle = 5.0 / 6.0 * PI;

    st.y += tan(angle) * 0.5;

    vec2 n = N(angle);
    float d = dot(st - vec2(0.5, 0.0), n);
    st -= n * max(0.0, d) * 2.0;

    n = N(2.0 / 3.0 * PI);
    float scale = 1.0;
    st.x += 0.5;

    for (int i = 0; i < 5; i++) {
        st *= 3.0;
        scale *= 3.0;
        st.x -= 1.5;

        st.x = abs(st.x);
        st.x -= 0.5;
        st -= n * min(0.0, dot(st, n)) * 2.0;
    }

    d = length(st - vec2(clamp(st.x, -1.0, 1.0), 0.0));
    st /= scale;

    // color.rg += st;
    color += d;

    frag_color = vec4(color, 1);
}
