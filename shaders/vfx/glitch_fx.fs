/*{
    "DESCRIPTION": "Audio reaactive glitch effects",
    "CREDIT": "by julesyoungberg",
    "ISFVSN": "2.0",
    "CATEGORIES": [ "Glitch" ],
    "INPUTS": [
        {
            "NAME": "inputImage",
            "TYPE": "image"
        },
        {
            "NAME": "fft_texture",
            "TYPE": "audioFFT",
            "MAX": 32
        },
        {
            "NAME": "slice_amount",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 1.0,
            "DEFAULT": 0.0
        },
        {
            "NAME": "slice_sensitivity",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 2.0,
            "DEFAULT": 1.0
        },
        {
            "NAME": "color_shake_amount",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 1.0,
            "DEFAULT": 0.0
        },
        {
            "NAME": "color_shake_sensitivity",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 2.0,
            "DEFAULT": 1.0
        }
    ]
}*/

// based on ImageGlitcher by airtight
// https://www.shadertoy.com/view/MtXBDs

#define SPECTRUM_SIZE 32

vec3 image_color(in vec2 coord) {
    return IMG_NORM_PIXEL(inputImage, fract(coord)).rgb;
}

float rand21(vec2 p) {
    return fract(sin(dot(p.xy, vec2(12.9898, 78.233))) * 43758.5453);
}

float rand_range(in vec2 seed, in float mn, in float mx) {
    return mn + rand21(seed) * (mx - mn);
}

float spectrum_strength(float start, float end) {
    float sum = 0.0;
    for (float i = start; i < end; i++) {
        sum += log(IMG_NORM_PIXEL(fft_texture, vec2(i / SPECTRUM_SIZE, 0)).x +
                   1.0);
    }
    return sum / (end - start);
}

void main() {
    vec2 st = isf_FragNormCoord;

    vec3 color = image_color(st);

    float t = floor(TIME * 0.5 * 60.0);

    // offset slices horizontally according to treble
    float max_offset = slice_amount + spectrum_strength(SPECTRUM_SIZE * 0.5, SPECTRUM_SIZE) *
                       2.0 * slice_sensitivity;
    for (float i = 0.0; i < max_offset * 20.0; i++) {
        // get random start and end y coords
        float slice_y = rand21(vec2(t, 3679.0 + i));
        float slice_h = rand21(vec2(t, 4582.0 + i)) * 0.25;
        // if we are inside the range shift the slice
        if (step(slice_y, st.y) - step(fract(slice_y + slice_h), st.y) == 1.0) {
            // get random horizontal shift
            float offset =
                rand_range(vec2(t, 6824.0 + i), -max_offset, max_offset);
            color = image_color(fract(vec2(st.x + offset, st.y)));
        }
    }

    // calculate color shift according to bass
    float max_color_offset = color_shake_amount + 
        spectrum_strength(0, SPECTRUM_SIZE * 0.5) * 0.02 * color_shake_sensitivity;
    vec2 color_offset =
        vec2(rand_range(vec2(t, 6794.0), -max_color_offset, max_color_offset),
             rand_range(vec2(t, 9382.0), -max_color_offset, max_color_offset));
    vec3 shifted_color = image_color(st + color_offset);

    // shift a random channel
    float rnd = rand21(vec2(t, 8379.0));
    if (rnd < 0.33) {
        color.r = shifted_color.r;
    } else if (rnd < 0.66) {
        color.g = shifted_color.g;
    } else {
        color.b = shifted_color.b;
    }

    vec2 st2 = abs(st * 2.0 - 1.0);
    vec2 border = 1.0 - smoothstep(vec2(0.95), vec2(1.0), st2);
    color *= mix(0.2, 1.0, border.x * border.y);

    gl_FragColor = vec4(color, 1.0);
}
