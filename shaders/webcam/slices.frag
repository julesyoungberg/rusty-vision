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

// based on Slices webcam by stanvanoers
// https://www.shadertoy.com/view/MlG3Wz

//@import util/noise

float noise2(in vec2 p);

vec3 webcam_color(in vec2 coord) {
    return texture(usampler2D(webcam, webcam_sampler), fract(coord)).xyz / 255.0;
}

float get_spectrum(float i) {
    return texture(sampler2D(spectrum, spectrum_sampler), vec2(fract(i), 0)).x;
}

void main() {
    vec2 st = uv * 0.5 + 0.5;

    // assign the pixel to a slice
    const float slices = 10.0;
    float slice = floor(st.y * slices);
    float s = slice / slices;

    // get a randomish value for this slice
    float n = noise2(vec2(slice, 0.0));

    // compute shift as combo of sin wave and spectral intensity
    const float intensity = 0.1;
    float shift = sin(n * time) * intensity - 0.05;
    shift *= log(1.0 + get_spectrum(s) * exp(s)) * 0.2;
    st.x += shift;

    vec3 color = webcam_color(st);
    
	frag_color = vec4(color, 1.0);
}
