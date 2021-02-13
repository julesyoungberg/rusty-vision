#version 450

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 frag_color;

layout(set = 0, binding = 0) uniform GeneralUniforms {
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

vec2 N(float angle) {
    return vec2(sin(angle), cos(angle));
}

void main() {
    vec2 st = uv;
    st.y *= resolution.y / resolution.x;
    st *= 2.0;
    vec3 color = vec3(0);

    st.x = abs(st.x);
    st.y += tan(5.0 / 6.0 * 3.1415) * 0.5;
    vec2 n = N(5.0 / 6.0 * 3.1415);
    float d = dot(st - vec2(0.5, 0.0), n);
    st -= n * max(d, 0.0) * 2.0;

    n = N(sin(time * 0.1) * 2.0 / 3.0 * 3.1415);
    float scale = 1.0;
    st.x += 0.5; // compensate for -= 1.5
    for (int i = 0; i < 4; i++) {
        st *= 3.0;
        scale *= 3.0;
        st.x -= 1.5;

        st.x = abs(st.x);
        st.x -= 0.5;
        st -= n * min(dot(st, n), 0.0) * 2.0;
    }

    // d = length(st - vec2(clamp(st.x, -1.0, 1.0), 0));
    // color += smoothstep(1.0 / resolution.y, 0.0, d / scale);
    st /= scale;
    color = texture(sampler2D(image1, image_sampler), mod(st - time * 0.05, 1.0)).xyz;

    frag_color = vec4(color, 1.0);
}
