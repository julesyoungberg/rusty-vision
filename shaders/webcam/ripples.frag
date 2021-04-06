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

// based on RippleCam by sleep
// https://www.shadertoy.com/view/4djGzz

//@import util/palette

vec3 palette(in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d);

vec3 webcam_color(in vec2 coord) {
    return texture(usampler2D(webcam, webcam_sampler), coord).xyz / 255.0;
}

// Simple circular wave function
float wave(vec2 pos, float freq, float numWaves, vec2 center) {
	float d = length(pos - center);
	d = log(1.0 + exp(d));
	float w = 0.3 / (1.0 + 20.0 * d * d) * sin(2.0 * 3.1415 * (-numWaves * d + time * freq));
    return w;
}

float get_spectrum(float i) {
    return texture(sampler2D(spectrum, spectrum_sampler), vec2(fract(i), 0)).x;
}

// This height map combines a couple of waves
float height(vec2 pos) {
	float w = wave(pos, 2.0, 10.0, vec2(0.0, -1.0));
    w *= get_spectrum(0.2) * 5.0 + 0.4;
    w += wave(pos, 3.0, 20.0, vec2(-1.0, 1.0)) * (get_spectrum(0.6) * 10.0 + 0.1);
    w += wave(pos, 3.0, 20.0, vec2(1.0, 1.0)) * (get_spectrum(0.6) * 10.0 + 0.1);
	return w;
}

// Discrete differentiation
vec2 normal(vec2 pos) {
	return 	vec2(height(pos - vec2(0.01, 0)) - height(pos), 
                 height(pos - vec2(0, 0.01)) - height(pos));
}

void main() {
    vec2 st = uv * 0.5 + 0.5;

    vec3 color = vec3(0.0);
    
    vec2 n = normal(uv);
    color = webcam_color(st + n);
    n *= 2.0;
    color.r += webcam_color(vec2(st.x + n.x, st.y)).r;
    color.g += webcam_color(vec2(st.x, st.y + n.y)).g;
    color.b += webcam_color(st - n).b;
    color /= 3.0;

	frag_color = vec4(color, 1.0);
}

