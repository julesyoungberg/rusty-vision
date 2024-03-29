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
            "NAME": "sensitivity",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 1.0,
            "DEFAULT": 0.5
        },
        {
            "NAME": "iterations",
            "TYPE": "float",
            "MIN": 5,
            "MAX": 20,
            "DEFAULT": 10
        },
        {
            "NAME": "speed",
            "TYPE": "float",
            "MIN": -0.1,
            "MAX": 0.1,
            "DEFAULT": 0.02
        },
        {
            "NAME": "zoom",
            "TYPE": "float",
            "MIN": 0.01,
            "MAX": 0.1,
            "DEFAULT": 0.07
        }
    ]
}*/

#define PI 3.14159265359

vec3 hsv2rgb(vec3 c) {
    vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
    return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}

mat2 rot(float a) {
    vec2 v = sin(vec2(1.57, 0.0) + a);
    return mat2(v, -v.y, v.x);
}

// based on Fractal Thingy by Klems
// https://www.shadertoy.com/view/Mt3Szr
float hex_dist(vec2 p) {
#define MULT1 (1.0 / tan(PI / 3.0))
#define MULT2 (1.0 / sin(PI / 3.0))
    float dx = abs(p.x);
    float dy = abs(p.y);
    return max(dx + dy * MULT1, max(dx, dy * MULT2));
}

vec4 fractal(in vec2 p) {
    float scale = 1.0;
    float alias_base = 1.0 / RENDERSIZE.y;

    float alpha = 0.0;
    vec3 color = vec3(0.0);
    float t = TIME * speed;

    for (float i = 0.0; i < iterations; i++) {
        float s = 2.0;

        // repeate axis
        p = 1.0 - abs(s * fract(p - 0.5) - s * 0.5);

        // fold
        float theta = (i + 1.0) * PI * 0.125;
        theta = t * i;
        p *= rot(theta);

        scale *= s;

        if (i < 4.0)
            continue;

        // pattern
        vec2 uv = abs(p);
        float hex_pattern = abs(hex_dist(uv) - 0.7);
        float circle_pattern = length(uv) - 0.2;
        float line_pattern = min(uv.x, uv.y);
        float mesh_pattern = min(circle_pattern, line_pattern);
        float pattern = min(hex_pattern, mesh_pattern);
        float alias = alias_base * 0.5 * scale;
        float f = smoothstep(0.1 + alias, 0.1, pattern) * 0.4 +
                  smoothstep(0.22, 0.11, pattern) * 0.6;

        // pulse
        float r = length(isf_FragNormCoord * 2.0 - 1.0) / 0.707106;
        float t = mod(TIME * 1.5, (iterations - 4.0) * 2.0) - i;
        r = (r + 1.0 - t) * step(r * 0.5, 1.0);
        r = smoothstep(0.0, 0.8, r) - smoothstep(0.9, 1.0, r);

        // mix colors
        vec3 c = vec3(smoothstep(0.06 + alias, 0.06, pattern));
        vec3 hue = hsv2rgb(vec3(TIME * 0.03 + i * 0.15, 0.7, 1.0));
        c = c * hue;
        c += c * r * 1.5;

        float spec_strength = log(
            IMG_NORM_PIXEL(fft_texture, vec2((i - 3.0) / iterations, 0.0)).x +
            1.0);
        float strength =
            clamp(mix(1.0 - sensitivity, 1.0, spec_strength), 0.0, 1.5);

        // front to back compositing
        color = (1.0 - alpha) * c * strength + color;
        alpha = (1.0 - alpha) * f * strength + alpha;
    }

    return vec4(color, alpha);
}

void main() {
    vec2 st = isf_FragNormCoord * 2.0 - 1.0;
    st.x *= RENDERSIZE.x / RENDERSIZE.y;

    vec3 color = vec3(0.0);

    // st += vec2(0.4487, 0.17567) * (TIME + 10.3312);
    st *= zoom;

    vec4 frac = fractal(st);

    // mix fractal with background
    color = mix(vec3(0.0), frac.rgb, frac.a);
    // vignette
    vec2 uv = isf_FragNormCoord * 2.0 - 1.0;
    color = mix(color, vec3(0.0), dot(st, st) * 0.5);

    gl_FragColor = vec4(color, 1);
}
