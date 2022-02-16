/*{
    "DESCRIPTION": "Metaballs lavalamp simulation",
    "CREDIT": "by julesyoungberg",
    "ISFVSN": "2.0",
    "CATEGORIES": [ "GENERATOR" ],
    "INPUTS": [
        {
            "NAME": "color1",
            "TYPE": "color",
            "DEFAULT": [
                1.0,
                1.0,
                1.0,
                1.0
            ]
        },
        {
            "NAME": "color2",
            "TYPE": "color",
            "DEFAULT": [
                0.0,
                0.1,
                0.1,
                1.0
            ]
        },
        {
            "NAME": "metaball_range",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 0.5,
            "DEFAULT": 0.1
        },
        {
            "NAME": "point_range",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 1.0,
            "DEFAULT": 0.5
        },
        {
            "NAME": "noise_displacement",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 1.0,
            "DEFAULT": 0.01
        },
        {
            "NAME": "noise_amount",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 1.0,
            "DEFAULT": 1.0
        },
        {
            "NAME": "a_x",
            "TYPE": "float",
            "MIN": -1.0,
            "MAX": 1.0,
            "DEFAULT": 0.5
        },
        {
            "NAME": "a_y",
            "TYPE": "float",
            "MIN": -1.0,
            "MAX": 1.0,
            "DEFAULT": 0.7
        },
        {
            "NAME": "b_x",
            "TYPE": "float",
            "MIN": -1.0,
            "MAX": 1.0,
            "DEFAULT": 0.2
        },
        {
            "NAME": "b_y",
            "TYPE": "float",
            "MIN": -1.0,
            "MAX": 1.0,
            "DEFAULT": -0.3
        },
        {
            "NAME": "c_x",
            "TYPE": "float",
            "MIN": -1.0,
            "MAX": 1.0,
            "DEFAULT": 0.3
        },
        {
            "NAME": "c_y",
            "TYPE": "float",
            "MIN": -1.0,
            "MAX": 1.0,
            "DEFAULT": 0.7
        },
        {
            "NAME": "d_x",
            "TYPE": "float",
            "MIN": -1.0,
            "MAX": 1.0,
            "DEFAULT": 0.9
        },
        {
            "NAME": "d_y",
            "TYPE": "float",
            "MIN": -1.0,
            "MAX": 1.0,
            "DEFAULT": 0.6
        },
        {
            "NAME": "noise_scale",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 50.0,
            "DEFAULT": 2.0
        },
        {
            "NAME": "noise_displacement_speed",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 1.0,
            "DEFAULT": 0.5
        },
        {
            "NAME": "noise_speed",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 1.0,
            "DEFAULT": 0.5
        },
        {
            "NAME": "distance_cutoff",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 1.0,
            "DEFAULT": 0.1
        }
    ]
}*/

#define TAU 6.28318530718

//	Simplex 3D Noise
//	by Ian McEwan, Ashima Arts
//
vec4 permute(vec4 x) { return mod(((x * 34.0) + 1.0) * x, 289.0); }
vec4 taylorInvSqrt(vec4 r) { return 1.79284291400159 - 0.85373472095314 * r; }

float snoise(vec3 v) {
    const vec2 C = vec2(1.0 / 6.0, 1.0 / 3.0);
    const vec4 D = vec4(0.0, 0.5, 1.0, 2.0);

    // First corner
    vec3 i = floor(v + dot(v, C.yyy));
    vec3 x0 = v - i + dot(i, C.xxx);

    // Other corners
    vec3 g = step(x0.yzx, x0.xyz);
    vec3 l = 1.0 - g;
    vec3 i1 = min(g.xyz, l.zxy);
    vec3 i2 = max(g.xyz, l.zxy);

    //  x0 = x0 - 0. + 0.0 * C
    vec3 x1 = x0 - i1 + 1.0 * C.xxx;
    vec3 x2 = x0 - i2 + 2.0 * C.xxx;
    vec3 x3 = x0 - 1. + 3.0 * C.xxx;

    // Permutations
    i = mod(i, 289.0);
    vec4 p = permute(permute(permute(i.z + vec4(0.0, i1.z, i2.z, 1.0)) + i.y +
                             vec4(0.0, i1.y, i2.y, 1.0)) +
                     i.x + vec4(0.0, i1.x, i2.x, 1.0));

    // Gradients
    // ( N*N points uniformly over a square, mapped onto an octahedron.)
    float n_ = 1.0 / 7.0; // N=7
    vec3 ns = n_ * D.wyz - D.xzx;

    vec4 j = p - 49.0 * floor(p * ns.z * ns.z); //  mod(p,N*N)

    vec4 x_ = floor(j * ns.z);
    vec4 y_ = floor(j - 7.0 * x_); // mod(j,N)

    vec4 x = x_ * ns.x + ns.yyyy;
    vec4 y = y_ * ns.x + ns.yyyy;
    vec4 h = 1.0 - abs(x) - abs(y);

    vec4 b0 = vec4(x.xy, y.xy);
    vec4 b1 = vec4(x.zw, y.zw);

    vec4 s0 = floor(b0) * 2.0 + 1.0;
    vec4 s1 = floor(b1) * 2.0 + 1.0;
    vec4 sh = -step(h, vec4(0.0));

    vec4 a0 = b0.xzyw + s0.xzyw * sh.xxyy;
    vec4 a1 = b1.xzyw + s1.xzyw * sh.zzww;

    vec3 p0 = vec3(a0.xy, h.x);
    vec3 p1 = vec3(a0.zw, h.y);
    vec3 p2 = vec3(a1.xy, h.z);
    vec3 p3 = vec3(a1.zw, h.w);

    // Normalise gradients
    vec4 norm =
        taylorInvSqrt(vec4(dot(p0, p0), dot(p1, p1), dot(p2, p2), dot(p3, p3)));
    p0 *= norm.x;
    p1 *= norm.y;
    p2 *= norm.z;
    p3 *= norm.w;

    // Mix final noise value
    vec4 m = max(0.6 - vec4(dot(x0, x0), dot(x1, x1), dot(x2, x2), dot(x3, x3)),
                 0.0);
    m = m * m;
    return 42.0 *
           dot(m * m, vec4(dot(p0, x0), dot(p1, x1), dot(p2, x2), dot(p3, x3)));
}

vec3 rand3(vec3 p) {
    return fract(sin(vec3(dot(p, vec3(127.1, 311.7, 264.9)),
                          dot(p, vec3(269.5, 183.3, 491.5)),
                          dot(p, vec3(27.17, 112.61, 57.53)))) *
                 43758.5453);
}

vec3 get_point(vec3 coord) {
    vec3 point = rand3(coord);
    point = sin(TIME * 0.5 + TAU * point) * 0.5;
    return point;
}

void main() {
    vec2 st = isf_FragNormCoord;
    st -= 0.5;
    st.x *= RENDERSIZE.x / RENDERSIZE.y;

    vec3 color = vec3(0.0);
    float metaball = 0.0;
    float range = metaball_range;

    vec2 diff1 = st - get_point(vec3(0.3, 0.7, 0.0)).xy * point_range;
    metaball += range * range / dot(diff1, diff1);

    vec2 diff2 = st - get_point(vec3(0.4, 0.7, 0.0)).xy * point_range;
    metaball += range * range / dot(diff2, diff2);

    vec2 diff3 = st - get_point(vec3(0.4, 0.8, 0.0)).xy * point_range;
    metaball += range * range / dot(diff3, diff3);

    vec2 diff4 = st - get_point(vec3(0.5, 0.6, 0.0)).xy * point_range;
    metaball += range * range / dot(diff4, diff4);

    float angle =
        snoise(vec3(st * noise_scale, TIME * noise_displacement_speed)) * TAU *
        0.5;
    vec2 displacement = vec2(sin(angle), cos(angle)) * noise_displacement;
    vec3 ball_color =
        mix(color1.rgb, color2.rgb,
            (snoise(vec3((st + displacement) * 5.0, TIME * noise_speed)) * 0.5 +
             0.5) *
                noise_amount);
    color = mix(color, ball_color,
                smoothstep(distance_cutoff, distance_cutoff + 0.01, metaball));

    gl_FragColor = vec4(color, 1.0);
}
