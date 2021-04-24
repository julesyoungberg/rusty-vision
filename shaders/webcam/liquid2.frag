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

// based on RippleCam by sleep
// https://www.shadertoy.com/view/4djGzz

const vec3 LIGHT_POS = vec3(0.5, 0.5, -1.0);

vec3 webcam_color(in vec2 coord) {
    vec2 c = vec2(coord.x, 1.0 - coord.y);
    return texture(sampler2D(webcam, webcam_sampler), fract(c)).rgb;
}

float get_spectrum(float i) {
    return texture(sampler2D(spectrum, spectrum_sampler), vec2(fract(i), 0)).x;
}

// This height map combines a couple of waves
float height(vec2 st) {
	float s1 = get_spectrum(0.3) * 2.0 + 0.1;
    float s2 = get_spectrum(0.6) * 2.0 + 0.1;
    float t = time;

    float shift = sin(st.y * 10.0 + t + cos(st.y * st.x + t)) * s1;
    shift *= sin(st.x * 15.0 + t * 1.387) * s2;
    return shift;
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
