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

//@import util/snoise

float snoise(vec3 v);

// based on Noisy Mirror by teadrinker
// https://www.shadertoy.com/view/lssSDH

vec3 webcam_color(in vec2 coord) {
    return texture(usampler2D(webcam, webcam_sampler), coord).xyz / 255.0;
}

float get_spectrum(float i) {
    return texture(sampler2D(spectrum, spectrum_sampler), vec2(fract(i), 0)).x;
}

const float scale_div = 4.;
const float scale_divt = 2.1;
const float sc1 = 1.0 / scale_div;
const float sc2 = sc1 / scale_div;
const float sc3 = sc2 / scale_div;
const float sc1t = 1.0 / scale_divt;
const float sc2t = sc1t / scale_divt;
const float sc3t = sc2t / scale_divt;
float fbm(vec3 v) {
	return 1.0 * 0.5 * snoise(v * vec3(sc3, sc3, sc3t)) * get_spectrum(0.2) * 2.0 + 
		   0.4 * 0.25 * snoise(v * vec3(sc2, sc2, sc2t)) * get_spectrum(0.4) * 2.0 + 
		   0.15 * 0.125 * snoise(v * vec3(sc1, sc1, sc1t)) * get_spectrum(0.6) * 2.0;
}

void main() {
    vec2 st = uv * 0.5 + 0.5;

    vec3 color = vec3(0.0);

    float noise_val1 = fbm(vec3(st * 80.0, time * 2.0));
    float noise_val2 = fbm(vec3(st * 80.0, time * 1.7 + 300.0));
    
    const float mag = 0.55;
    st += vec2(mag * 0.2 * noise_val1, mag * 0.21 * noise_val2);

    color = webcam_color(st);
    
	frag_color = vec4(color, 1.0);
}
