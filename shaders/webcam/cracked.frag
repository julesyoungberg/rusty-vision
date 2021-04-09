#version 450

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 frag_color;

layout(set = 0, binding = 0) uniform GeneralUniforms {
    vec2 mouse;
    vec2 resolution;
    float time;
    int mouse_down;
};

layout(set = 1, binding = 0) uniform sampler webcam_sampler;
layout(set = 1, binding = 1) uniform utexture2D webcam;
layout(set = 1, binding = 2) uniform WebcamUniforms {
    vec2 video_size;
};

layout(set = 2, binding = 0) uniform sampler spectrum_sampler;
layout(set = 2, binding = 1) uniform texture2D spectrum;

// based on cracked 2 by FMS_Cat
// https://www.shadertoy.com/view/XdBSzW
// doesn't work - not sure why

#define PI 3.14159265359

//@import util/rand

float rand21(vec2 p);

vec3 webcam_color(in vec2 coord) {
    return texture(usampler2D(webcam, webcam_sampler), coord).xyz / 255.0;
}

float rnd(vec2 s) {
    return 1.0 - 2.0 * fract(sin(s.x * 253.13 + s.y * 341.41) * 589.19);
}

void main() {
    vec2 st = uv;
    st.y *= resolution.y / resolution.x;

    vec3 color = vec3(0.0);

    vec2 v = vec2(1e3);   
    vec2 v2 = vec2(1e4);
    vec2 center = vec2(0.1, -0.5);

    for (float c = 0.0; c < 90.0; c++) {
        float angle = floor(rnd(vec2(c, 387.44)) * 16.0) * PI * 0.4 - 0.5;
        float dist = pow(rnd(vec2(c, 78.21)), 2.0) * 0.5;
        vec2 vc = vec2(
            center.x + cos(angle) * dist + rnd(vec2(c, 349.3)) * 7e-3,
            center.y + sin(angle) * dist + rnd(vec2(c, 912.7)) * 7e-3
        );
    
        if (length(vc - st) < length(v - st)) {
            v2 = v;
            v = vc;
        } else if (length(vc - st) < length(v2 - st)) {
            v2 = vc;
        }
    }

    float col = abs(length(dot(st - v, normalize(v - v2))) 
        - length(dot(st - v2, normalize(v - v2)))) 
        + 0.002 * length(st - center);
    col = 7e-4 / col;
    
    if (length(v - v2) < 4e-3) {
        col = 0.0;
    }

    if (col < 0.3) {
        col = 0.0;
    }

    color = webcam_color(uv * 0.5 + 0.5 + rnd(v) * 0.02);

	frag_color = col * vec4(vec3(1.0 - color), 1.0) + (1.0 - col) * vec4(color, 1.0);
}
