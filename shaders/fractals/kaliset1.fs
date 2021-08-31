/*{
    "DESCRIPTION": "",
    "CREDIT": "by julesyoungberg",
    "ISFVSN": "2.0",
    "CATEGORIES": [ "GENERATOR" ],
    "INPUTS": [
        {
            "NAME": "fft_texture",
            "TYPE": "audioFFT"
        },
        {
            "NAME": "iterations",
            "TYPE": "float",
            "MIN": 1,
            "MAX": 50,
            "DEFAULT": 32
        },
        {
            "NAME": "sensitivity",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 1.0,
            "DEFAULT": 0.5
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
            "NAME": "strength",
            "TYPE": "float",
            "MIN": 1.0,
            "MAX": 16.0,
            "DEFAULT": 8.0
        },
        {
            "NAME": "power",
            "TYPE": "float",
            "MIN": 1.0,
            "MAX": 10.0,
            "DEFAULT": 2.5
        },
        {
            "NAME": "speed",
            "TYPE": "float",
            "MIN": -1.0,
            "MAX": 1.0,
            "DEFAULT": 0.1
        }
    ]
}*/

// based on Simplicity by JoshP
// https://www.shadertoy.com/view/lslGWr

// IQ's palette generator:
// https://www.iquilezles.org/www/articles/palettes/palettes.htm
vec3 palette(in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d) {
    return a + b * cos(6.28318 * (c * t + d));
}

float tri_wave(float x) { return abs(2.0 * fract(x) - 1.0); }

vec4 kaliset(in vec3 z) {
    float accum = 0.0;
    float prev = 0.0;
    float tw = 0.0;
    float t = TIME * speed;

    for (float i = 0.0; i < iterations; i++) {
        z = abs(z) / dot(z, z);
        z += vec3(-0.6 + sin(t) * 0.3, -0.4 + sin(t * 2.7 + 2.1) * 0.3,
                  -1.5 + sin(t * 0.7 + 1.3) * 0.01);

        float mag = dot(z, z);
        float w = exp(-i / 7.0);
        accum += w * exp(-strength * pow(abs(mag - prev), power));
        tw += w;
    }

    return vec4(z, max(0.0, 5.0 * accum / tw - 0.7));
}

void main() {
    vec2 st = isf_FragNormCoord * 2.0 - 1.0;
    st.x *= RENDERSIZE.x / RENDERSIZE.y;

    vec3 p = vec3(st, (tri_wave(TIME * 0.003) - 0.5) * 2.0);

    vec4 res = kaliset(p);
    vec2 r = res.xy;
    float d = res.w;

    vec3 color = palette(
        d, vec3(0.5, 0.5, 0.5), vec3(0.5, 0.5, 0.5), vec3(1.0, 1.0, 1.0),
        fract(
            vec3(log(IMG_NORM_PIXEL(fft_texture, vec2(0.7, 0)).x * sensitivity +
                     rOffset),
                 log(IMG_NORM_PIXEL(fft_texture, vec2(0.4, 0)).x * sensitivity +
                     gOffset),
                 log(IMG_NORM_PIXEL(fft_texture, vec2(0.1, 0)).x * sensitivity +
                     bOffset))));

    // edge fade
    vec2 uv = isf_FragNormCoord * 2.0 - 1.0;
    float v = (1.0 - exp((abs(uv.x) - 1.0) * 6.0)) *
              (1.0 - exp((abs(uv.y) - 1.0) * 6.0));
    color *= mix(0.4, 1.0, v);

    gl_FragColor = vec4(color * color, 1);
}
