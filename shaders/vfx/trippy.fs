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

// based on lsdSheetGenerator by netgrind
// https://www.shadertoy.com/view/ltf3Dl

#define TAU 6.28318530718

vec3 hsv2rgb(vec3 c) {
    vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
    return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}

vec3 rgb2hsv(in vec3 c) {
    vec4 K = vec4(0.0, -1.0 / 3.0, 2.0 / 3.0, -1.0);
    vec4 p = c.g < c.b ? vec4(c.bg, K.wz) : vec4(c.gb, K.xy);
    vec4 q = c.r < p.x ? vec4(p.xyw, c.r) : vec4(c.r, p.yzx);
    float d = q.x - min(q.w, q.y);
    float e = 1.0e-10;
    return vec3(abs(q.z + (q.w - q.y) / (6.0 * d + e)), d / (q.x + e), q.x);
}

vec3 image_color(in vec2 coord) {
    return IMG_NORM_PIXEL(input_image, fract(coord)).rgb;
}

float get_spectrum(float i) {
    return log(IMG_NORM_PIXEL(fft_texture, vec2(fract(i), 0)).x + 1.0);
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
    vec2 st = isf_FragNormCoord;

    vec3 color = vec3(1.0);
    const int loops = 2;

    for (int i = 0; i < loops; i++) {
        // transform space
        st *= 2.0;
        st -= 1.0;
        float angle = -TIME * 0.4 * i;
        st *= mat2(cos(angle), -sin(angle), sin(angle), cos(angle));
        st = abs(st);

        // blend iteration colors
        color = (cos(abs(color - image_color(fract(st))) * TAU) + 1.0) * 0.5;
    }

    color = hue_shift(color, TIME * 0.5);

    gl_FragColor = vec4(color, 1.0);
}
