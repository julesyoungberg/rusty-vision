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

void main() {
    vec2 st = uv * 0.5 + 0.5;;
    st.y *= resolution.y / resolution.x;
    st *= 2.0;

    vec3 color = vec3(0.0);

    float t1 = time * 1.13;
    float t2 = time * 0.9;

    for (float i = 1.0; i < 4.0; i += 1.0) {
        vec2 p = st;
        p.x += 0.75 / i * sin(i * PI * st.y + t1 * 0.1);
        p.y += 0.35 / i * cos(i * PI * st.x + t2);
        st = p;
    }

    color = texture(sampler2D(image1, image_sampler), fract(st)).xyz;
    frag_color = vec4(color, 1);
}
