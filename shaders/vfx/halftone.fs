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

// based on Halftone Effect by VIDVOX
// https://editor.isf.video/shaders/5e7a801e7c113618206deafc

#define SPECTRUM_SIZE 32
#define TAU 6.28318530718
#define SPECTRUM_SIZE 32

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

float noise_hash2(vec2 p) {
    p = 50.0 * fract(p * 0.3183099 + vec2(0.71, 0.113));
    return -1.0 + 2.0 * fract(p.x * p.y * (p.x + p.y));
}

float noise21(in vec2 p) {
    vec2 i = floor(p);
    vec2 f = fract(p);
    vec2 u = f * f * (3.0 - 2.0 * f);

    return mix(mix(noise_hash2(i + vec2(0.0, 0.0)),
                   noise_hash2(i + vec2(1.0, 0.0)), u.x),
               mix(noise_hash2(i + vec2(0.0, 1.0)),
                   noise_hash2(i + vec2(1.0, 1.0)), u.x),
               u.y);
}

float rand21(vec2 p) {
    return fract(sin(dot(p.xy, vec2(12.9898, 78.233))) * 43758.5453);
}

float rand_range(in vec2 seed, in float mn, in float mx) {
    return mn + rand21(seed) * (mx - mn);
}

vec4 image_color(in vec2 coord) { return IMG_NORM_PIXEL(input_image, coord); }

float spectrum_strength(float start, float end) {
    float sum = 0.0;
    for (float i = start; i < end; i++) {
        sum += log(IMG_NORM_PIXEL(fft_texture, vec2(i / SPECTRUM_SIZE, 0)).x +
                   1.0);
    }
    return sum / (end - start);
}

float dot_pattern(in vec2 st, float angle, float scale, in vec2 center) {
    float s = sin(angle * TAU);
    float c = cos(angle * TAU);
    vec2 p = (st - center) * RENDERSIZE * scale * mat2(c, -s, s, c);
    return sin(p.x) * sin(p.y) * 4.0 *
           (sin(angle * TAU + scale * TIME * 6.0) * 0.5 + 1.0);
}

float circle_pattern(in vec2 st, float angle, float scale, in vec2 center) {
    float d =
        distance(st * RENDERSIZE, center * RENDERSIZE) * max(scale, 0.001);
    return sin(d + angle * TAU - TIME * 6.0) * 4.0;
}

float line_pattern(in vec2 st, float angle, float scale, in vec2 center) {
    float s = sin(angle * TAU * 0.5);
    float c = cos(angle * TAU * 0.5);
    vec2 p = st * RENDERSIZE * scale * mat2(c, -s, s, c);
    return (center.x + sin(p.y + center.y * TAU + TIME * 12.0)) * 4.0;
}

float[SPECTRUM_SIZE] get_spectrum() {
    float spec[SPECTRUM_SIZE];
    for (int i = 0; i < SPECTRUM_SIZE; i++) {
        spec[i] = log(
            IMG_NORM_PIXEL(fft_texture, vec2(float(i) / SPECTRUM_SIZE, 0)).x +
            1.0);
    }
    return spec;
}

float scaled(float v) { return log(v + 1.0); }

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
    vec2 st = isf_FragNormCoord;

    float spec[SPECTRUM_SIZE] = get_spectrum();
    int max_bin = arg_max(spec);

    // fetch convolution samples
    vec3 offset = vec3(1.0 / RENDERSIZE, 0.0);
    vec4 center_color = image_color(st);
    vec4 left_color = image_color(st - offset.xz);
    vec4 right_color = image_color(st + offset.xz);
    vec4 top_color = image_color(st + offset.zy);
    vec4 bottom_color = image_color(st - offset.zy);
    vec4 tl_color = image_color(st + vec2(-offset.x, offset.y));
    vec4 tr_color = image_color(st + offset.xy);
    vec4 bl_color = image_color(st - offset.xy);
    vec4 br_color = image_color(st + vec2(offset.x, -offset.y));

    // apply convolution filter
    float sharpness = 10.0;
    vec4 color_sum = left_color + right_color + top_color + bottom_color +
                     tl_color + tr_color + bl_color + br_color;
    // vec4 color = center_color + sharpness * (8.0 * center_color - color_sum);
    vec4 color = (center_color + color_sum) / 9.0;

    // convert to hsv and discretize domain
    float divisions = 4.0;
    vec3 hsv = rgb2hsv(color.rgb);
    vec3 discrete = floor(hsv * divisions + 0.5) / divisions;

    float average = (hsv.r + hsv.g + hsv.b) / 3.0;
    float a = average * 10.0 - 5.0;
    int patternType =
        int(2.99 * noise21(discrete.xy * 5.0 +
                           (float(max_bin) / SPECTRUM_SIZE) + TIME * 0.25));
    float angle = 0.0;
    float scale = 1.0;
    vec3 final = vec3(0);
    if (patternType == 0) {
        final = vec3(a + dot_pattern(st, angle + discrete.x, scale * discrete.y,
                                     vec2(0.5)));
    } else if (patternType == 1) {
        final = vec3(a + circle_pattern(st, angle + discrete.x,
                                        scale * discrete.y, vec2(0.5)));
    } else if (patternType == 2) {
        final = vec3(a + line_pattern(st, angle + discrete.x,
                                      scale * discrete.y, vec2(0.5)));
    }

    color.rgb = mix(color.rgb * final, final, 0.0);
    gl_FragColor = color;
}