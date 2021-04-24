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
layout(set = 1, binding = 1) uniform texture2D webcam;
layout(set = 1, binding = 2) uniform WebcamUniforms {
    vec2 video_size;
};

layout(set = 2, binding = 0) uniform sampler spectrum_sampler;
layout(set = 2, binding = 1) uniform texture2D spectrum;

#define TAU 6.28318530718

vec3 webcam_color(in vec2 coord) {
    vec2 c = vec2(coord.x, 1.0 - coord.y);
    return texture(sampler2D(webcam, webcam_sampler), fract(c)).rgb;
}

// based on ngMir8 by netgrind
// https://www.shadertoy.com/view/XtlSzX
void main() {
    vec2 st = uv * 0.5 + 0.5;
    
    vec3 color = webcam_color(st);

    float t = time;
    float d = mix(0.01, 0.1, texture(sampler2D(spectrum, spectrum_sampler), vec2(0.1, 0.0)).x);

    const float taps = 6.0;

    for (float i = 0.0; i < TAU; i += TAU / taps) {
        float a = i + t;
        vec3 color2 = webcam_color(vec2(st.x + cos(a) * d, st.y + sin(a) * d));
        color = min(color, color2);
    }
    
	frag_color = vec4(color, 1.0);
}
