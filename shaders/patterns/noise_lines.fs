/*{
    "DESCRIPTION": "Lines displaced by noise",
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
            "NAME": "noise_octaves",
            "TYPE": "float",
            "MIN": 1.0,
            "MAX": 6.0,
            "DEFAULT": 2.0
        },
        {
            "NAME": "noise_speed",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 0.5,
            "DEFAULT": 0.15
        },
        {
            "NAME": "shift_speed",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 0.5,
            "DEFAULT": 0.2
        }
    ]
}*/

#define PI 3.14159265359

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
        f += scale * noise31(vec3(p, TIME * noise_speed));
        // p *= m * (2.0 + noise_hash2(p) * 0.01);
        scaling += scale;
        scale *= 0.5;
    }

    f /= scaling;
    return f;
}


float line_dist(vec2 p, vec2 a, vec2 b) {
    vec2 pa = p - a;
    vec2 ba = b - a;
    float t = clamp(dot(pa, ba) / dot(ba, ba), 0.0, 1.0);
    return length(pa - ba * t);
}

float line(vec2 p, vec2 a, vec2 b) {
    float d = line_dist(p, a, b);
    float m = smoothstep(0.002, 0.0, d);
    return m;
}

float displaced_line(in vec2 st, float line_y, float x_len) {
    float width = x_len * 0.5;
    float center = x_len * 0.5;
    float start = center - width * 0.5;
    float end = center + width * 0.5;
    float shift = TIME * shift_speed;
    float shifted_x = mod(st.x - shift, x_len);
    float amount = smoothstep(start, center, shifted_x) - smoothstep(center, end, shifted_x);
    st.y -= 0.2 * fbm(st * vec2(3.0, 4.0)) * amount;
    return line(st, vec2(0.0, line_y), vec2(x_len, line_y));
}

void main() {
    vec2 st = isf_FragNormCoord;
    float ratio = RENDERSIZE.x / RENDERSIZE.y;
    st.x *= ratio;

    vec3 color = vec3(0.0);

    float num_lines = 7.0;
    float line_spacing = 0.05;
    float lines_width = (num_lines - 1.0) * line_spacing;
    float start = 0.5 - lines_width * 0.5;
    float end = 0.5 + lines_width * 0.5;

    for (float y = start; y <= end; y += line_spacing) {
        color += displaced_line(st, y, ratio);
    }

    // vec3 color = vec3(fbm(st));
    // vec3 color = palette(fbm(st), color_config1.rgb, color_config2.rgb,
    //                      color_config3.rgb, color_config4.rgb);
    gl_FragColor = vec4(color, 1);
}
