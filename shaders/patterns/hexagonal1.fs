/*{
    "DESCRIPTION": "",
    "CREDIT": "by julesyoungberg",
    "ISFVSN": "2.0",
    "CATEGORIES": [ "GENERATOR" ],
    "INPUTS": [
        {
            "NAME": "color_config",
            "TYPE": "color",
            "DEFAULT": [
                0.4,
                0.1,
                1.0,
                1.0
            ]
        },
        {
            "NAME": "fft_texture",
            "TYPE": "audioFFT"
        },
        {
            "NAME": "scale",
            "TYPE": "float",
            "MIN": 1.0,
            "MAX": 30.0,
            "DEFAULT": 10.0
        },
        {
            "NAME": "sensitivity",
            "TYPE": "float",
            "MIN": 1.0,
            "MAX": 5.0,
            "DEFAULT": 2.0
        },
        {
            "NAME": "threshold",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 1.0,
            "DEFAULT": 0.3
        },
        {
            "NAME": "color_speed",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 0.5,
            "DEFAULT": 0.1
        },
        {
            "NAME": "pulse_speed",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 5.0,
            "DEFAULT": 1.0
        },
        {
            "NAME": "frequency_density",
            "TYPE": "float",
            "MIN": 0.01,
            "MAX": 1.0,
            "DEFAULT": 0.1
        }
    ]
}*/

vec3 hsv2rgb(vec3 c) {
    vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
    return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
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

vec4 hex_coords(in vec2 st) {
    vec2 r = vec2(1, sqrt(3));
    vec2 h = r * 0.5;

    vec2 a = mod(st, r) - h;
    vec2 b = mod(st - h, r) - h;

    vec2 gv = length(a) < length(b) ? a : b;

    float x = atan(gv.x, gv.y);
    float y = 0.5 - hex_dist(gv);
    vec2 id = st - gv;

    return vec4(x, y, id);
}

void main() {
    vec2 st = isf_FragNormCoord * 2.0 - 1.0;
    st.y *= RENDERSIZE.y / RENDERSIZE.x;

    vec3 color = vec3(0);

    st *= scale;

    vec4 coords = hex_coords(st);
    vec2 gv = coords.xy;
    vec2 id = coords.zw;

    float i = dot(id, id);

    float intensity =
        log(IMG_NORM_PIXEL(fft_texture, vec2(fract(i * frequency_density), 0))
                .x +
            1.0) *
        sensitivity;

    float d = smoothstep(0.01, 0.03, gv.y * sin(i + TIME * pulse_speed));

    color = palette(sin(i + TIME * color_speed), vec3(0.5, 0.5, 0.5),
                    vec3(0.5, 0.5, 0.5), vec3(1.0, 1.0, 1.0), color_config.xyz);

    color *= d * mix(0.0, 1.0, smoothstep(threshold, 1.0, intensity));

    // float edge_dist = 0.5 - hex_dist(gv);
    // color = mix(vec3(0), color, smoothstep(0.05, 0.06, edge_dist));

    gl_FragColor = vec4(color, 1);
}
