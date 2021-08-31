/*{
    "DESCRIPTION": "",
    "CREDIT": "by julesyoungberg",
    "ISFVSN": "2.0",
    "CATEGORIES": [ "GENERATOR" ],
    "INPUTS": [
        {
            "NAME": "fft_texture",
            "TYPE": "audioFFT",
            "MAX": 32
        },
        {
            "NAME": "iterations",
            "TYPE": "float",
            "MIN": 1,
            "MAX": 20,
            "DEFAULT": 12
        },
        {
            "NAME": "rOffset",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 2.0,
            "DEFAULT": 0.7
        },
        {
            "NAME": "gOffset",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 2.0,
            "DEFAULT": 0.4
        },
        {
            "NAME": "bOffset",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 2.0,
            "DEFAULT": 0.1
        },
        {
            "NAME": "pulse",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 1.0,
            "DEFAULT": 0.05
        }
    ]
}*/

#define SPECTRUM_SIZE 32

float rand21(vec2 p) {
    return fract(sin(dot(p.xy, vec2(12.9898, 78.233))) * 43758.5453);
}

vec2 rand2(vec2 p) {
    return fract(
        sin(vec2(dot(p, vec2(127.1, 311.7)), dot(p, vec2(269.5, 183.3)))) *
        43758.5453);
}

// IQ's palette generator:
// https://www.iquilezles.org/www/articles/palettes/palettes.htm
vec3 palette(in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d) {
    return a + b * cos(6.28318 * (c * t + d));
}

// based on Illustrated Equations by sben
// https://www.shadertoy.com/view/MtBGDW0
// and Circuits by Kali
// https://www.shadertoy.com/view/XlX3Rj
void main() {
    vec2 st = isf_FragNormCoord * 2.0 - 1.0;
    st.x *= RENDERSIZE.x / RENDERSIZE.y;

    vec3 color = vec3(0.0);

    float strength = log(IMG_NORM_PIXEL(fft_texture, vec2(0.04, 0)).x + 1.0);

    st *= 2.0;
    st *= mix(1.0, 1.0 - pulse, strength);

    vec2 p = abs(st * 2.0);
    vec2 ab = vec2(2.0 - p.x);
    float t = TIME;

    // orbit traps
    float min_comp = 1000.0;
    float min_mag = min_comp;
    float last_stable = 0.0;

    for (float i = 0.0; i < iterations; i++) {
        // fractal equation
        ab += p + cos(length(p) - t) * sin(t * 0.1);
        p.y += sin(ab.x - p.x - t) * 0.5;
        p.x += sin(ab.y - t) * 0.5;
        p -= p.x + p.y;
        p += (st.y + cos(st.x) * sin(t * 0.267)) * sin(t * 0.345);
        ab += vec2(p.y);

        // update orbit traps
        float mag = length(p);
        float w = 0.1;
        float m_comp = clamp(abs(min(p.x, p.y)), w - mag, abs(mag - w));
        min_comp = min(m_comp, min_comp);
        min_mag = min(mag * 0.1, min_mag);
        last_stable =
            max(last_stable, i * (1.0 - abs(sign(min_comp - m_comp))));
    }

    p /= 30.0;

    float id = p.x * 2.0 + p.y;

    // get fractal color
    color = palette(
        id * 2.0, vec3(0.5, 0.5, 0.5), vec3(0.5, 0.5, 0.5), vec3(1.0, 1.0, 1.0),
        fract(
            vec3(log(IMG_NORM_PIXEL(fft_texture, vec2(rOffset, 0)).x + 1.0),
                 log(IMG_NORM_PIXEL(fft_texture, vec2(gOffset, 0)).x + 1.0),
                 log(IMG_NORM_PIXEL(fft_texture, vec2(bOffset, 0)).x + 1.0))));

    // carve out design
    last_stable += 1.0;
    float intensity = 0.01;
    float width = intensity * last_stable * 2.0;
    float circ = pow(max(0.0, width - min_mag) / width, 6.0);
    float shape = max(pow(max(0.0, width - min_comp) / width, 0.25), circ);
    color *= shape;

    gl_FragColor = vec4(sqrt(color), 1);
}
