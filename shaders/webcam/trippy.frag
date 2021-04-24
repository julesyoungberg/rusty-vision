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

// based on lsdSheetGenerator by netgrind
// https://www.shadertoy.com/view/ltf3Dl
#define TAU 6.28318530718

//@import util/hsv2rgb
//@import util/rgb2hsv

vec3 hsv2rgb(vec3 c);
vec3 rgb2hsv(in vec3 c);

vec3 webcam_color(in vec2 coord) {
    vec2 c = vec2(coord.x, 1.0 - coord.y);
    return texture(sampler2D(webcam, webcam_sampler), fract(c)).rgb;
}

float get_spectrum(float i) {
    return texture(sampler2D(spectrum, spectrum_sampler), vec2(fract(i), 0)).x;
}

float spectrum_strength(float start, float end) {
    float sum = 0.0;
    for (float i = start; i < end; i += 1.0 / 32.0) {
        sum += get_spectrum(i);
    }
    return sum / (end - start);
}

vec3 hue_shift(vec3 color, float shift) {
    vec3 hsv = rgb2hsv(color);
    hsv.r = fract(hsv.r + shift);
    float colors = 2.0;
    hsv.b *= get_spectrum(hsv.r);
    return hsv2rgb(hsv);
}

void main() {
    vec2 st = uv * 0.5 + 0.5;
    
    vec3 color = vec3(1.0);
    const int loops = 2;

    for (int i = 0; i < loops; i++) {
        // transform space
        st *= 2.0;
        st -= 1.0;
        float angle = -time * 0.4 * i;
        st *= mat2(cos(angle), -sin(angle), sin(angle), cos(angle));
        st = abs(st);

        // blend iteration colors
        color = (cos(abs(color - webcam_color(fract(st))) * TAU) + 1.0) * 0.5;
    }

    color = hue_shift(color, time * 0.5);
    
	frag_color = vec4(color, 1.0);
}
