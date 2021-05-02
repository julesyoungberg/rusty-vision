/*{
    "DESCRIPTION": "Audio reaactive glitch effects",
    "CREDIT": "by julesyoungberg",
    "ISFVSN": "2.0",
    "CATEGORIES": [ "FX" ],
    "INPUTS": [
        {
            "NAME": "fft_texture",
            "TYPE": "audioFFT"
        },
        {
            "NAME": "input_image",
            "TYPE": "image"
        }
    ]
}*/

// based on RippleCam by sleep
// https://www.shadertoy.com/view/4djGzz

vec3 image_color(in vec2 coord) {
    return IMG_NORM_PIXEL(input_image, fract(coord)).rgb;
}

float get_spectrum(float i) {
    return log(IMG_NORM_PIXEL(fft_texture, vec2(fract(i), 0)).x + 1.0);
}

// Simple circular wave function
float wave(vec2 pos, float freq, float numWaves, vec2 center) {
    float d = length(pos - center);
    d = log(1.0 + exp(d));
    float w = 0.3 / (1.0 + 20.0 * d * d) *
              sin(2.0 * 3.1415 * (-numWaves * d + TIME * freq));
    return w;
}

// This height map combines a couple of waves
float height(vec2 pos) {
    float w = wave(pos, 2.0, 10.0, vec2(0.0, -1.0));
    w *= get_spectrum(0.2) * 5.0 + 0.4;
    w += wave(pos, 3.0, 20.0, vec2(-1.0, 1.0)) *
         (get_spectrum(0.6) * 10.0 + 0.1);
    w +=
        wave(pos, 3.0, 20.0, vec2(1.0, 1.0)) * (get_spectrum(0.6) * 10.0 + 0.1);
    return w;
}

// Discrete differentiation
vec2 normal(vec2 pos) {
    return vec2(height(pos - vec2(0.01, 0)) - height(pos),
                height(pos - vec2(0, 0.01)) - height(pos));
}

void main() {
    vec2 st = isf_FragNormCoord;

    vec3 color = vec3(0.0);

    vec2 n = normal(isf_FragNormCoord * 2.0 - 1.0);
    color = image_color(st + n);
    n *= 2.0;
    color.r += image_color(vec2(st.x + n.x, st.y)).r;
    color.g += image_color(vec2(st.x, st.y + n.y)).g;
    color.b += image_color(st - n).b;
    color /= 3.0;

    gl_FragColor = vec4(color, 1.0);
}
