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

#define PI 3.14159265359

// https://www.shadertoy.com/view/MtKXDR
vec2 kaleidoscope(vec2 st) {
    float a = atan(st.y, st.x);
    float r = pow(length(st), 0.9);
    float p = sin(2.0 * PI * time * 0.02);
    float q = 2.0 * PI / 9.0;
    a = abs(mod(a, q) - 0.5 * q);
    float factor = pow(r, 1.3) * 0.1;
    return vec2(cos(a), sin(a)) * factor;
}

vec2 transform(vec2 st) {
    float a = time * 0.02;
    vec2 v;
    v.x = st.x * cos(a) - st.y * sin(a) - 0.3 * sin(a);
    v.y = st.x * sin(a) + st.y * cos(a) + 0.3 * cos(a);
    return v;
}

vec4 scene(vec2 st) {
    return texture(sampler2D(image1, image_sampler), mod(transform(st) * 2.0, 1.0));
}

void main() {
    vec2 st = uv;
    st.y *= resolution.y / resolution.x;
	frag_color = scene(kaleidoscope(st));
}
