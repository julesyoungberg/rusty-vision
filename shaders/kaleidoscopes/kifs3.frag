#version 450

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 frag_color;

layout(set = 0, binding = 0) uniform GeneralUniforms {
    vec2 mouse;
    vec2 resolution;
    float time;
    int mouse_down;
};

layout(set = 1, binding = 0) uniform sampler image_sampler;
layout(set = 1, binding = 1) uniform texture2D image1;
layout(set = 1, binding = 2) uniform texture2D image2;
layout(set = 1, binding = 2) uniform ImageUniforms {
    vec2 image1_size;
    vec2 image2_size;
};

#define PI 3.14159265359
#define PHI 1.6180339887

vec2 N(float angle) {
    return vec2(sin(angle), cos(angle));
}

float sdBox(in vec2 p, in vec2 b) {
    vec2 d = abs(p) - b;
    return length(max(d, 0.0)) + min(max(d.x, d.y), 0.0);
}

float square(in vec2 p, in vec2 b) {
    float angle = PI * 0.25 * time * -0.5;
    float c = cos(angle);
    float s = sin(angle);
    return sdBox(p * mat2(c, -s, s, c), b);
}

void main() {
    vec2 st = uv;
    st.y *= resolution.y / resolution.x;
    st *= 3.0;
    vec3 color = vec3(0);
    vec2 size = vec2(0.5);

    float scale = 1.0;
    float dist = 100.0;

    for (int i = 0; i < 10; i++) {
        float angle = time * 0.1 + i;
        float c = cos(angle);
        float s = sin(angle);
        st *= mat2(c, -s, s, c);

        bool even = mod(i, 2) == 0;
        dist = min(dist, square(st, size) * scale);

        if (even) {
            st.x = abs(st.x);
        } else {
            st.y = abs(st.y);
        }

        st *= PHI;
        scale /= PHI;

        if (even) {
            st.x -= PHI * sqrt(2);
        } else {
            st.y -= PHI * sqrt(2);
        }
    }

    st *= scale;
    color = texture(sampler2D(image1, image_sampler), mod(st - time * 0.05, 1.0)).xyz;
    // color += 1.0 - sign(dist);

    frag_color = vec4(color, 1.0);
}
