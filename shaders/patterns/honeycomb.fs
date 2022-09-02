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
            "NAME" : "audio_reactive",
            "TYPE" : "bool",
            "DEFAULT" : 0
        },
        {
            "NAME": "scale",
            "TYPE": "float",
            "MIN": 0.25,
            "MAX": 1.75,
            "DEFAULT": 1.0
        },
        {
            "NAME": "thickness",
            "TYPE": "float",
            "MIN": 0.05,
            "MAX": 0.2,
            "DEFAULT": 0.07
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
            "DEFAULT": 4.0
        },
        {
            "NAME": "depth",
            "TYPE": "float",
            "MIN": 10.0,
            "MAX": 30.0,
            "DEFAULT": 20.0
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

// shane's hexagonal tiling (https://www.shadertoy.com/view/llSyDh)
vec4 get_hex(vec2 p) {
    vec4 hc = floor(vec4(p, p - vec2(0.5, 1)) / s.xyxy) + 0.5;
    vec4 h = vec4(p - hc.xy * s, p - (hc.zw + 0.5) * s);
    return (dot(h.xy, h.xy) < dot(h.zw, h.zw))
               ? vec4(h.xy, hc.xy)
               : vec4(h.zw, hc.zw + vec2(0.5, 1));
}

// IQ's palette generator:
// https://www.iquilezles.org/www/articles/palettes/palettes.htm
vec3 palette(in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d) {
    return a + b * cos(6.28318 * (c * t + d));
}

float hex_dist(in vec2 p) {
    p = abs(p);
    return max(dot(p, normalize(vec2(1.0, sqrt(3)))), p.x);
}

float rand21(vec2 p) {
    return fract(sin(dot(p.xy, vec2(12.9898, 78.233))) * 43758.5453);
}

// draws 1 layer of the psudeo-3d effect
vec3 layer(vec2 st, float n) {
    vec4 hex = get_hex(st);
    vec2 gv = hex.xy;
    vec2 id = hex.zw;

    float dist = 0.5 - hex_dist(gv);

    vec3 color = vec3(0);

    float i = fract(id.x * id.y * 0.05 + n);

    float d = smoothstep(thickness, 0.0, dist);

    float intensity = audio_reactive ?
        log(IMG_NORM_PIXEL(fft_texture, vec2(fract(i * freq_diversity), 0.0))
                .x +
            1.0) *
        sensitivity : 1.0;

    float shade = d * i * color_diversity *
                  max(rand21(id) - 1.0 + density, 0.0) * intensity;

    color =
        shade *
        palette(sin(i * color_diversity + TIME * color_speed + color_offset),
                vec3(0.5, 0.5, 0.5), vec3(0.5, 0.5, 0.5), vec3(1.0, 1.0, 1.0),
                color_config.rgb);

    return color;
}

void main() {
    vec2 st = isf_FragNormCoord * 2.0 - 1.0;
    st.x *= RENDERSIZE.x / RENDERSIZE.y;

    st *= scale;

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
        float fade = smoothstep(0.0, 0.2, z) * smoothstep(0.8, 0.6, z);
        st *= rot;
        color += layer(st * size + i * vec2(20.0, 27.0), i) * fade;
    }

    // float gradient_strength = IMG_NORM_PIXEL(fft_texture, vec2(0.1, 0)).x;
    // color -= max(gradient * gradient_strength, 0.0);

    gl_FragColor = vec4(color, 1);
}
