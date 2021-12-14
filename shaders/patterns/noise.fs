/*{
    "DESCRIPTION": "Simple FBM color pattern",
    "CREDIT": "by julesyoungberg",
    "ISFVSN": "2.0",
    "CATEGORIES": [ "GENERATOR" ],
    "INPUTS": [
        {
            "NAME": "color_config1",
            "TYPE": "color",
            "DEFAULT": [
                0.5,
                0.5,
                0.5,
                1.0
            ]
        },
        {
            "NAME": "color_config2",
            "TYPE": "color",
            "DEFAULT": [
                0.5,
                0.5,
                0.5,
                1.0
            ]
        },
        {
            "NAME": "color_config3",
            "TYPE": "color",
            "DEFAULT": [
                1.0,
                1.0,
                1.0,
                1.0
            ]
        },
        {
            "NAME": "color_config4",
            "TYPE": "color",
            "DEFAULT": [
                0.1,
                0.3,
                0.6,
                1.0
            ]
        },
        {
            "NAME": "grid_scale",
            "TYPE": "float",
            "MIN": 1.0,
            "MAX": 10.0,
            "DEFAULT": 2.0
        },
        {
            "NAME": "noise_octaves",
            "TYPE": "float",
            "MIN": 1.0,
            "MAX": 6.0,
            "DEFAULT": 2.0
        },
        {
            "NAME": "speed",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 1.0,
            "DEFAULT": 0.15
        }
    ]
}*/

// IQ's palette generator:
// https://www.iquilezles.org/www/articles/palettes/palettes.htm
vec3 palette(in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d) {
    return a + b * cos(6.28318 * (c * t + d));
}

float noise_hash2(vec2 p) {
    p = 50.0 * fract(p * 0.3183099 + vec2(0.71, 0.113));
    return -1.0 + 2.0 * fract(p.x * p.y * (p.x + p.y));
}

float noise_hash3(vec3 p) {
    p = fract(p * 0.3183099 + .1);
    p *= 17.0;
    return fract(p.x * p.y * p.z * (p.x + p.y + p.z));
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

float noise31(in vec3 x) {
    vec3 i = floor(x);
    vec3 f = fract(x);
    f = f * f * (3.0 - 2.0 * f);

    return mix(mix(mix(noise_hash3(i + vec3(0, 0, 0)),
                       noise_hash3(i + vec3(1, 0, 0)), f.x),
                   mix(noise_hash3(i + vec3(0, 1, 0)),
                       noise_hash3(i + vec3(1, 1, 0)), f.x),
                   f.y),
               mix(mix(noise_hash3(i + vec3(0, 0, 1)),
                       noise_hash3(i + vec3(1, 0, 1)), f.x),
                   mix(noise_hash3(i + vec3(0, 1, 1)),
                       noise_hash3(i + vec3(1, 1, 1)), f.x),
                   f.y),
               f.z);
}

float fbm(in vec2 p) {
    const mat2 m = mat2(0.8, 0.6, -0.6, 0.8);
    float f = 0.0;
    float scale = 0.5;
    float scaling = 0.0;

    for (float i = 0.0; i < noise_octaves; i++) {
        f += scale * noise31(vec3(p, TIME * speed));
        p *= m * (2.0 + noise_hash2(p) * 0.01);
        scaling += scale;
        scale *= 0.5;
    }

    f /= scaling;
    return f;
}

void main() {
    vec2 st = isf_FragNormCoord * 2.0 - 1.0;
    st *= grid_scale;
    vec3 color = palette(fbm(st), color_config1.rgb, color_config2.rgb,
                         color_config3.rgb, color_config4.rgb);
    gl_FragColor = vec4(color, 1);
}
