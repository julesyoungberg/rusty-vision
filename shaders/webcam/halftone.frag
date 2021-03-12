#version 450

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 frag_color;

layout(set = 0, binding = 0) uniform GeneralUniforms {
    vec2 mouse;
    vec2 resolution;
    float time;
    int mouse_down;
};

layout(set = 1, binding = 0) uniform sampler spectrum_sampler;
layout(set = 1, binding = 1) uniform texture2D spectrum;

layout(set = 2, binding = 0) uniform sampler webcam_sampler;
layout(set = 2, binding = 1) uniform utexture2D webcam;
layout(set = 2, binding = 2) uniform WebcamUniforms {
    vec2 video_size;
};

// based on Halftone Effect by VIDVOX
// https://editor.isf.video/shaders/5e7a801e7c113618206deafc

#define SPECTRUM_SIZE 32
#define TAU 6.28318530718
#define SPECTRUM_SIZE 32

//@import util/hsv2rgb
//@import util/noise
//@import util/rand
//@import util/rgb2hsv

vec3 hsv2rgb(vec3 c);
float noise2(in vec2 p);
float rand21(vec2 p);
float rand_range(in vec2 seed, in float mn, in float mx);
vec3 rgb2hsv(in vec3 c);

vec4 webcam_color(vec2 p) {
    return texture(usampler2D(webcam, webcam_sampler), p) / 255.0;
}

float spectrum_strength(float start, float end) {
    float sum = 0.0;
    for (float i = start; i < end; i++) {
        sum += texture(sampler2D(spectrum, spectrum_sampler), vec2(i / SPECTRUM_SIZE, 0)).x;
    }
    return sum / (end - start);
}

float dot_pattern(in vec2 st, float angle, float scale, in vec2 center) {
    float s = sin(angle * TAU);
    float c = cos(angle * TAU);
    vec2 p = (st - center) * resolution * scale * mat2(c, -s, s, c);
    return sin(p.x) * sin(p.y) * 4.0 * (sin(angle * TAU + scale * time * 6.0) * 0.5 + 1.0);
}

float circle_pattern(in vec2 st, float angle, float scale, in vec2 center) {
    float d = distance(st * resolution, center * resolution) * max(scale, 0.001);
    return sin(d + angle * TAU - time * 6.0) * 4.0;
}

float line_pattern(in vec2 st, float angle, float scale, in vec2 center) {
    float s = sin(angle * TAU * 0.5);
    float c = cos(angle * TAU * 0.5);
    vec2 p = st * resolution * scale * mat2(c, -s, s, c);
    return (center.x + sin(p.y + center.y * TAU + time * 12.0)) * 4.0;
}

float[SPECTRUM_SIZE] get_spectrum() {
    float spec[SPECTRUM_SIZE];
    for (int i = 0; i < SPECTRUM_SIZE; i++) {
        spec[i] = texture(sampler2D(spectrum, spectrum_sampler), vec2(float(i) / SPECTRUM_SIZE, 0)).x;
    }
    return spec;
}

float scaled(float v) {
    return log(v + 1.0);
}

int arg_max(float array[SPECTRUM_SIZE]) {
    float mx = scaled(array[0]);
    int ix = 0;

    for (int i = 1; i < SPECTRUM_SIZE; i++) {
        float v = scaled(array[i]);
        if (v > mx) {
            mx = v;
            ix = i;
        }
    }

    return ix;
}

void main() {
    vec2 st = uv * 0.5 + 0.5;

    float spec[SPECTRUM_SIZE] = get_spectrum();
    int max_bin = arg_max(spec);

    // fetch convolution samples
    vec3 offset = vec3(1.0 / resolution, 0.0);
    vec4 center_color = webcam_color(st);
    vec4 left_color = webcam_color(st - offset.xz);
    vec4 right_color = webcam_color(st + offset.xz);
    vec4 top_color = webcam_color(st + offset.zy);
    vec4 bottom_color = webcam_color(st - offset.zy);
    vec4 tl_color = webcam_color(st + vec2(-offset.x, offset.y));
    vec4 tr_color = webcam_color(st + offset.xy);
    vec4 bl_color = webcam_color(st - offset.xy);
    vec4 br_color = webcam_color(st + vec2(offset.x, -offset.y));

    // apply convolution filter
    float sharpness = 10.0;
    vec4 color_sum = left_color + right_color + top_color + bottom_color + tl_color + tr_color + bl_color + br_color;
    // vec4 color = center_color + sharpness * (8.0 * center_color - color_sum);
    vec4 color = (center_color + color_sum) / 9.0;
    
    // convert to hsv and discretize domain
    float divisions = 4.0;
    vec3 hsv = rgb2hsv(color.rgb);
    vec3 discrete = floor(hsv * divisions + 0.5) / divisions;

    float average = (hsv.r + hsv.g + hsv.b) / 3.0;
    float a = average * 10.0 - 5.0;
    int patternType = int(2.99 * noise2(discrete.xy * 5.0 + (float(max_bin) / SPECTRUM_SIZE) + time * 0.25));
    float angle = 0.0;
    float scale = 1.0;
    vec3 final = vec3(0);
    if (patternType == 0) {
        final = vec3(a + dot_pattern(st, angle + discrete.x, scale * discrete.y, vec2(0.5)));
    } else if (patternType == 1) {
        final = vec3(a + circle_pattern(st, angle + discrete.x, scale * discrete.y, vec2(0.5)));
    } else if (patternType == 2) {
        final = vec3(a + line_pattern(st, angle + discrete.x, scale * discrete.y, vec2(0.5)));
    }
    
    color.rgb = mix(color.rgb * final, final, 0.0);
	frag_color = color;
}