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

//@import util/palette

vec3 palette(in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d) ;

vec3 webcam_color(in vec2 coord) {
    return texture(usampler2D(webcam, webcam_sampler), coord).xyz / 255.0;
}

// based on RippleCam by sleep
// https://www.shadertoy.com/view/4djGzz
void main() {
    vec2 st = uv * 0.5 + 0.5;
    // st += normal(uv, time);
    
    const vec2 params = vec2(2.5, 10.0);
    
    vec3 color = webcam_color(st);

	frag_color = vec4(color, 1.0);
}

