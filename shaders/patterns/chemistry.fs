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
            "NAME": "color_config",
            "TYPE": "color",
            "DEFAULT": [
                0.2,
                0.4,
                0.6,
                1.0
            ]
        },
        {
            "NAME": "zoom_speed",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 0.5,
            "DEFAULT": 0.1
        },
        {
            "NAME": "rotation_speed",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 0.1,
            "DEFAULT": 0.01
        },
        {
            "NAME": "color_speed",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 0.5,
            "DEFAULT": 0.1
        },
        {
            "NAME": "color_offset",
            "TYPE": "float",
            "MIN": -3.14159265359,
            "MAX": 3.14159265359,
            "DEFAULT": 0.0
        },
        {
            "NAME": "n_layers",
            "TYPE": "float",
            "MIN": 1.0,
            "MAX": 8.0,
            "DEFAULT": 1.0
        },
        {
            "NAME": "depth",
            "TYPE": "float",
            "MIN": 10.0,
            "MAX": 20.0,
            "DEFAULT": 15.0
        },
        {
            "NAME": "sensitivity",
            "TYPE": "float",
            "MIN": 1.0,
            "MAX": 5.0,
            "DEFAULT": 2.0
        },
        {
            "NAME": "color_diversity",
            "TYPE": "float",
            "MIN": 1.0,
            "MAX": 10.0,
            "DEFAULT": 5.0
        },
        {
            "NAME": "freq_diversity",
            "TYPE": "float",
            "MIN": 1.0,
            "MAX": 10.0,
            "DEFAULT": 7.0
        },
        {
            "NAME": "density",
            "TYPE": "float",
            "MIN": 0.1,
            "MAX": 0.4,
            "DEFAULT": 0.3
        }
    ]
}*/

#define PI 3.14159265359
#define TAU 6.28318530718

const vec2 s = vec2(1, 1.7320508);

// IQ's palette generator:
// https://www.iquilezles.org/www/articles/palettes/palettes.htm
vec3 palette(in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d) {
    return a + b * cos(6.28318 * (c * t + d));
}

float rand21(vec2 p) {
    return fract(sin(dot(p.xy, vec2(12.9898, 78.233))) * 43758.5453);
}

float line_dist(vec2 p, vec2 a, vec2 b) {
    vec2 pa = p - a;
    vec2 ba = b - a;
    float t = clamp(dot(pa, ba) / dot(ba, ba), 0.0, 1.0);
    return length(pa - ba * t);
}

float line(vec2 p, vec2 a, vec2 b) {
    float d = line_dist(p, a, b);
    float m = smoothstep(0.02, 0.0, d);
    return m;
}

float polygon_edges(vec2 st, int edges, int start_edge, int end_edge) {
    float a = atan(st.x, st.y) + PI;
    float r = TAU / float(edges);
    float shifted = mod(a + r * 0.5, TAU);
    float edge_range = step(float(start_edge) * r, shifted) -
                       step(float(end_edge) * r, shifted);

    float d = cos(floor(0.5 + a / r) * r - a) * length(st) * edge_range;

    return max(smoothstep(0.42, 0.41, d) - smoothstep(0.39, 0.38, d), 0.0);
}

// draws 1 layer of the psudeo-3d effect
vec3 layer(vec2 st, float n) {
    vec3 color = vec3(0.0);

    color += polygon_edges(st, 6, 1, 4);

    return color;
}

void main() {
    vec2 st = isf_FragNormCoord * 2.0 - 1.0;
    st.x *= RENDERSIZE.x / RENDERSIZE.y;

    vec3 color = vec3(0.0);

    float gradient = st.y * zoom_speed;

    float zoom = TIME * zoom_speed;
    float r = TIME * rotation_speed;

    float s = sin(r);
    float c = cos(r);
    mat2 rot = mat2(c, -s, s, c);
    st *= rot;

    for (float i = 0.0; i < n_layers; i += 1.0) {
        float z = fract(i / n_layers + zoom);
        float size = mix(20.0, 1.0, z);
        float fade = smoothstep(0.0, 0.2, z) * smoothstep(1.0, 0.8, z);
        st *= rot;
        color += layer(st * size + i * vec2(20.0, 27.0), i) * fade;
    }

    // float gradient_strength = IMG_NORM_PIXEL(fft_texture, vec2(0.1, 0)).x;
    // color -= max(gradient * gradient_strength, 0.0);

    gl_FragColor = vec4(color, 1);
}
