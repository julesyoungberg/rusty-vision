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
            "NAME": "scale",
            "TYPE": "float",
            "MIN": 1.0,
            "MAX": 50.0,
            "DEFAULT": 20.0
        },
        {
            "NAME": "shimmer_speed",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 10.0,
            "DEFAULT": 6.0
        },
        {
            "NAME": "speed",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 5.0,
            "DEFAULT": 1.0
        },
        {
            "NAME": "sensitivity",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 1.0,
            "DEFAULT": 0.1
        },
        {
            "NAME": "color1",
            "TYPE": "color",
            "DEFAULT": [
                0.19,
                0.25,
                0.43,
                1.0
            ]
        },
        {
            "NAME": "color2",
            "TYPE": "color",
            "DEFAULT": [
                0.35,
                0.06,
                0.28,
                1.0
            ]
        },
        {
            "NAME": "positions",
            "TYPE": "color",
            "DEFAULT": [
                0.33,
                0.5,
                0.85,
                0.25
            ]
        },
        {
            "NAME": "shimmer_amount",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 1.0,
            "DEFAULT": 1.0
        }
    ]
}*/

#define AUDIO_REACTIVE 1
#define TAU 6.28318530718

const vec2 s = vec2(1, 1.7320508);

bool above_line(vec2 r, vec2 q, vec2 p) {
    return dot(vec2(q.y - r.y, r.x - q.x), q - p) > 0.0;
}

float line_dist(vec2 p, vec2 a, vec2 b) {
    vec2 pa = p - a;
    vec2 ba = b - a;
    float t = clamp(dot(pa, ba) / dot(ba, ba), 0.0, 1.0);
    return length(pa - ba * t);
}

// IQ's power curve function
// https://www.iquilezles.org/www/articles/functions/functions.htm
float power_curve(float x, float a, float b) {
    float k = pow(a + b, a + b) / (pow(a, a) * pow(b, b));
    return k * pow(x, a) * pow(1.0 - x, b);
}

// IQ's pulse function
// https://www.iquilezles.org/www/articles/functions/functions.htm
float pulse(float c, float w, float x) {
    x = abs(x - c);
    if (x > w)
        return 0.0;
    x /= w;
    return 1.0 - x * x * (3.0 - 2.0 * x);
}

float rand(float n) { return fract(n * 1183.5437 + .42); }

float rand21(vec2 p) {
    return fract(sin(dot(p.xy, vec2(12.9898, 78.233))) * 43758.5453);
}

vec2 rand2(vec2 p) {
    return fract(
        sin(vec2(dot(p, vec2(127.1, 311.7)), dot(p, vec2(269.5, 183.3)))) *
        43758.5453);
}

// shane's hexagonal tiling (https://www.shadertoy.com/view/llSyDh)
vec4 get_hex(vec2 p) {
    vec4 hc = floor(vec4(p, p - vec2(0.5, 1)) / s.xyxy) + 0.5;
    vec4 h = vec4(p - hc.xy * s, p - (hc.zw + 0.5) * s);
    return (dot(h.xy, h.xy) < dot(h.zw, h.zw))
               ? vec4(h.xy, hc.xy)
               : vec4(h.zw, hc.zw + vec2(0.5, 1));
}

vec2 get_point(vec2 id) { return sin(rand2(id) * TIME * speed) * 0.15; }

float line(vec2 p, vec2 a, vec2 b) {
    float d = line_dist(p, a, b);
    float m = smoothstep(0.02, 0.0, d);
    return m;
}

void main() {
    vec2 st = isf_FragNormCoord * 2.0 - 1.0;
    st.y *= RENDERSIZE.y / RENDERSIZE.x;
    st = st * 0.5 + 0.5;

    vec3 color = vec3(0);

    st *= scale;

    vec4 hex_coords = get_hex(st);
    vec2 gv = hex_coords.xy;
    vec2 id = hex_coords.zw;

    // color.rg = id * 0.05;

    vec2 points[7];
    vec2 coords[7];

    // collect neighboring points
    for (int i = 0; i <= 6; i++) {
        vec2 offset = vec2(0);
        if (i < 6.0) {
            float angle = float(i) * TAU / 6.0;
            float si = sin(angle);
            float co = cos(angle);
            offset = vec2(1.0, 0.0) * mat2(co, -si, si, co);
        }

        vec2 coord = get_hex(st + offset).zw;
        coords[i] = coord;
        points[i] = get_point(coord) + offset;
    }

    // find the current triangle
    bool l1 = above_line(points[6], points[0], gv);
    bool l2 = above_line(points[6], points[1], gv);
    bool l3 = above_line(points[6], points[2], gv);
    bool l4 = above_line(points[6], points[3], gv);
    bool l5 = above_line(points[6], points[4], gv);
    bool l6 = above_line(points[6], points[5], gv);

    int n1 = 0;
    int n2 = 0;

    // get neighboring coords for the current triangle
    if (l1 && !l2) {
        // top right
        n1 = 0;
        n2 = 1;
    } else if (l2 && !l3) {
        // top
        n1 = 1;
        n2 = 2;
    } else if (l3 && !l4) {
        // top left
        n1 = 2;
        n2 = 3;
    } else if (l4 && !l5) {
        // bottom left
        n1 = 3;
        n2 = 4;
    } else if (l5 && !l6) {
        // bottom
        n1 = 4;
        n2 = 5;
    } else if (l6 && !l1) {
        // bottom right
        n1 = 5;
        n2 = 0;
    }

    vec2 c1 = coords[n1];
    vec2 c2 = coords[n2];
    vec2 tri_coord = (id + c1 + c2) / 3.0;
    tri_coord /= scale;

    // color gradient
    float d1 = 1.0 - length(tri_coord - positions.xy) * 2.0;
    color = mix(vec3(0.0), color1.rgb, d1 * step(0.01, d1));
    float d2 = 1.0 - length(tri_coord - positions.zw) * 2.0;
    color = mix(color, color2.rgb, d2 * step(0.01, d2));

    // shimmer
    float dist = length(tri_coord) * 4.0;
    float t =
        dist - TIME * shimmer_speed + (tri_coord.x + tri_coord.y) * 2.0 - 10.0;
    float shine = mix(1.0, 1.0 + shimmer_amount, pulse(3.0, 2.0, mod(t, 16.8)));
    color *= shine;

    // randomly darkened tiles
    float darkness = rand21(tri_coord) * 0.5 + 0.5;
    color *= darkness;

    // randomly sparkling tiles
    vec2 rnd = rand2(tri_coord);
    float ti = rand(dot(rnd, rnd) * 0.1);
    float sparkle = 1.0;

    float loop = 30.0;
    float t2 = TIME * 0.5 + ti * loop;
    sparkle = mix(0.5, 2.0, max(0.0, power_curve(mod(t2, loop), 2.0, 1.0)));

    float intensity =
        log(IMG_NORM_PIXEL(fft_texture, vec2(fract(ti), 0)).x + 1.0) *
        sensitivity;
    sparkle += intensity + 1.0;
    color *= sparkle;

    // draw lines
    for (int i = 0; i < 6; i++) {
        color += line(gv, points[6], points[i]) * 0.03;
    }

    // correct center point
    float correction = length(gv - points[6]);
    color -= smoothstep(0.02, 0.0, correction) * 0.1;

    gl_FragColor = vec4(pow(color, vec3(1.5)), 1);
}
