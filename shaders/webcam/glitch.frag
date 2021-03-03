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

#define SPECTRUM_SIZE 32

//@import util/rand

float rand21(vec2 p);
float rand_range(in vec2 seed, in float mn, in float mx);

vec3 webcam_color(vec2 p) {
    return texture(usampler2D(webcam, webcam_sampler), p).xyz / 255.0;
}

float spectrum_strength(float start, float end) {
    float sum = 0.0;
    for (float i = start; i < end; i++) {
        sum += texture(sampler2D(spectrum, spectrum_sampler), vec2(i / SPECTRUM_SIZE, 0)).x;
    }
    return sum / (end - start);
}

// based on https://www.shadertoy.com/view/MtXBDs
void main() {
    vec2 st = uv * 0.5 + 0.5;

    vec3 color = webcam_color(st);

    float t = floor(time * 0.5 * 60.0);

    // offset slices horizontally according to treble
    float max_offset = spectrum_strength(SPECTRUM_SIZE * 0.5, SPECTRUM_SIZE) * 2.0;
    for (float i = 0.0; i < max_offset * 20.0; i++) {
        // get random start and end y coords
        float slice_y = rand21(vec2(t, 3679.0 + i));
        float slice_h = rand21(vec2(t, 4582.0 + i)) * 0.25;
        // if we are inside the range shift the slice
        if (step(slice_y, st.y) - step(fract(slice_y + slice_h), st.y) == 1.0) {
            // get random horizontal shift
            float offset = rand_range(vec2(t, 6824.0 + i), -max_offset, max_offset);
            color = webcam_color(fract(vec2(st.x + offset, st.y)));
        }
    }

    // calculate color shift according to bass
    float max_color_offset = spectrum_strength(0, SPECTRUM_SIZE * 0.5) * 0.02;
    vec2 color_offset = vec2(
        rand_range(vec2(t, 6794.0), -max_color_offset, max_color_offset),
        rand_range(vec2(t, 9382.0), -max_color_offset, max_color_offset)
    );
    vec3 shifted_color = webcam_color(st + color_offset);

    // shift a random channel
    float rnd = rand21(vec2(t, 8379.0));
    if (rnd < 0.33) {
        color.r = shifted_color.r;
    } else if (rnd < 0.66) {
        color.g = shifted_color.g;
    } else {
        color.b = shifted_color.b;
    }
    
	frag_color = vec4(color, 1.0);
}
