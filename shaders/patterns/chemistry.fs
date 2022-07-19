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

float circle(vec2 st) {
    float d = length(st);
    return smoothstep(0.05, 0.049, d);
}

mat2 make_rot(float angle) {
    float s = sin(angle);
    float c = cos(angle);
    return mat2(c, -s, s, c);
}

float benzene_ring(in vec2 st) {
    float shade = 0.0;

    shade += polygon_edges(st, 6, 0, 6);
    shade += polygon_edges(st * 1.25, 6, 0, 1);
    shade += polygon_edges(st * 1.25, 6, 2, 3);
    shade += polygon_edges(st * 1.25, 6, 4, 5);

    return shade;
}

// float hex_leg(in vec2 st, int edge, vec2 dotloc) {
//     float shade = 0.0;

//     shade += polygon_edges(st, 6, edge, edge + 1);
//     shade += circle(st + vec2())
// }

float left_leg1(in vec2 st) {
    float shade = 0.0;

    // st += vec2(0.7, -0.4);

    shade += polygon_edges(st + vec2(0.7, -0.4), 6, 0, 1);
    shade += circle(st + vec2(0.9, 0.0));

    return shade;
}

float left_leg2(in vec2 st) {
    float shade = 0.0;

    // st += vec2(0.7, 0.4);

    shade += polygon_edges(st + vec2(0.7, 0.4), 6, 5, 6);
    shade += circle(st + vec2(0.45, 0.78));

    return shade;
}

// @todo do this properly with trigonometry instead of estimating
float serotonin(vec2 st) {
    float shade = 0.0;

    st *= make_rot(TAU / 6.0 * 0.5);

    // main hex
    shade += benzene_ring(st);

    // HO leg
    shade += left_leg1(st);

    // main pentagon
    vec2 uv = st;
    uv *= 1.25;
    uv *= make_rot(-0.2);
    uv -= vec2(0.875, 0.3);
    shade += polygon_edges(uv, 5, 0, 5);
    shade += polygon_edges(uv * 1.25, 5, 3, 4);
    shade += circle(uv - vec2(0.3, -0.4));

    // partial hex
    uv = st;
    uv -= vec2(1.0);
    uv *= make_rot(0.6);
    uv -= vec2(0.0, -0.05);
    shade += polygon_edges(uv, 6, 1, 3);

    // final line
    uv -= vec2(-0.7, 0.4);
    shade += polygon_edges(uv, 6, 4, 5);
    shade += circle(uv - vec2(0.22, 0.42));

    return shade;
}

float dopamine(vec2 st) {
    float shade = 0.0;

    // main hex
    shade += benzene_ring(st);

    // HO leg
    shade += left_leg1(st);

    // OH leg
    shade += left_leg2(st);

    // partial hex
    vec2 uv = st - vec2(0.7, 0.4);
    shade += polygon_edges(uv, 6, 2, 4);

    // NH2 right leg
    uv -= vec2(0.7, 0.4);
    shade += polygon_edges(uv, 6, 2, 3);
    shade += circle(uv - vec2(-0.22, 0.42));

    return shade;
}

// draws 1 layer of the psudeo-3d effect
vec3 layer(vec2 st, float n) {
    vec3 color = vec3(0.0);

    float shade = 0.0;

    shade += serotonin(st - vec2(7.0, 5.0));
    shade += dopamine(st + vec2(5.0, 0.0));

    color += max(shade, 0.0);

    return color;
}

void main() {
    vec2 st = isf_FragNormCoord * 2.0 - 1.0;
    st.x *= RENDERSIZE.x / RENDERSIZE.y;

    vec3 color = vec3(0.0);

    float gradient = st.y * zoom_speed;

    float zoom = TIME * zoom_speed;
    float r = TIME * rotation_speed;

    mat2 rot = make_rot(r);
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
