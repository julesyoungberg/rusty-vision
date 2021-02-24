#version 450

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 frag_color;

layout(set = 0, binding = 0) uniform GeneralUniforms {
    vec2 mouse;
    vec2 resolution;
    float time;
};

layout(set = 1, binding = 0) uniform sampler image_sampler;
layout(set = 1, binding = 1) uniform texture2D image1;
layout(set = 1, binding = 2) uniform texture2D image2;
layout(set = 1, binding = 2) uniform ImageUniforms {
    vec2 image1_size;
    vec2 image2_size;
};

#define PI 3.14159265359

vec2 N(float angle) {
    return vec2(sin(angle), cos(angle));
}

void main() {
    vec2 st = uv;
    st.y *= resolution.y / resolution.x;
    st *= 1.5;
    vec3 color = vec3(0);

    st.x = abs(st.x);
    st.y += tan(5.0 / 6.0 * PI) * 0.5;
    vec2 n = N(5.0 / 6.0 * PI);
    float d = dot(st - vec2(0.5, 0.0), n);
    st -= n * max(d, 0.0) * 2.0;

    n = N(2.0 / 3.0 * PI); // sin(time * 0.0) * PI);
    float factor = 3.0; // (sin(time * 0.01) * 0.5 + 0.75) * 5.0;
    float shift = -1.5; // * sin(time * 0.05 - 1.5);
    float scale = 1.0;
    st.x -= shift / factor; // compensate for -= 1.5
    for (int i = 0; i < 4; i++) {
        st *= factor;
        scale *= factor;
        st.x += shift;

        st.x = abs(st.x);
        st.x -= 0.5;
        st -= n * min(dot(st, n), 0.0) * 2.0;
    }

    st /= scale;
    // d = length(st - vec2(clamp(st.x, -1.0, 1.0), 0));
    // color += smoothstep(1.0 / resolution.y, 0.0, d / scale);
    color = texture(sampler2D(image1, image_sampler), mod(st - time * 0.05, 1.0)).xyz;
    // color.rg += st * 0.1;

    frag_color = vec4(color, 1.0);
}
