/*{
    "DESCRIPTION": "FBM marble pattern generator based on the Book of Shaders",
    "CREDIT": "by julesyoungberg",
    "ISFVSN": "2.0",
    "CATEGORIES": [ "GENERATOR" ],
    "INPUTS": [
        {
            "NAME": "color1",
            "TYPE": "color",
            "DEFAULT": [0.05, 0.05, 0.1, 1.0]
        },
        {
            "NAME": "color2",
            "TYPE": "color",
            "DEFAULT": [1.0, 1.0, 1.0, 1.0]
        },
        {
            "NAME": "offset",
            "TYPE": "point2D"
        },
        {
            "NAME": "offsetA",
            "TYPE": "point2D"
        },
        {
            "NAME": "offsetB",
            "TYPE": "point2D"
        },
        {
            "NAME": "offsetC",
            "TYPE": "point2D"
        },
        {
            "NAME": "offsetD",
            "TYPE": "point2D"
        },
        {
            "NAME": "noise_scale",
            "TYPE": "float",
            "MIN": 1.0,
            "MAX": 20.0,
            "DEFAULT": 5.0
        },
        {
            "NAME": "line_thickness",
            "TYPE": "float",
            "MIN": 0.01,
            "MAX": 0.1,
            "DEFAULT": 0.03
        },
        {
            "NAME": "gain",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 1.0,
            "DEFAULT": 0.3
        },
        {
            "NAME": "lacunarity",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 3.0,
            "DEFAULT": 0.5
        },
        {
            "NAME": "scale1",
            "TYPE": "float",
            "MIN": 1.0,
            "MAX": 4.0,
            "DEFAULT": 2.0
        },
        {
            "NAME": "scale2",
            "TYPE": "float",
            "MIN": 1.0,
            "MAX": 4.0,
            "DEFAULT": 2.0
        },
        {
            "NAME": "noise_mode",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 1.99,
            "DEFAULT": 1.0
        },
        {
            "NAME": "octaves",
            "TYPE": "float",
            "MIN": 1.0,
            "MAX": 10.0,
            "DEFAULT": 2.0
        },
        {
            "NAME": "speed",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 2.0,
            "DEFAULT": 0.1
        }
    ]
}*/

//	Simplex 3D Noise
//	by Ian McEwan, Ashima Arts
//
vec4 permute(vec4 x) { return mod(((x * 34.0) + 1.0) * x, 289.0); }
vec4 taylorInvSqrt(vec4 r) { return 1.79284291400159 - 0.85373472095314 * r; }

float snoise(vec3 v) {
    const vec2 C = vec2(0.1666666667, 0.3333333333);
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
    float n_ = 0.1428571429; // 1.0/7.0; // N=7
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

float getNoiseVal(vec3 p) {
    float raw = snoise(p);

    // if (mirror == 1) {
    //     return abs(raw);
    // }

    return raw * 0.5 + 0.5;
}

float fbm(vec2 p) {
    float sum = 0.0;
    float freq = 1.0;
    float amp = 0.5;
    float prev = 1.0;
    vec3 v = vec3(p, TIME * speed);

    for (int i = 0; i < octaves; i++) {
        float n = getNoiseVal(v * freq);

        // if (invert == 1) {
        n = 1.0 - n;
        // }

        // if (sharpen == 1) {
        n = n * n;
        // }

        sum += n * amp;

        // if (scaleByPrev == 1) {
        //     sum += n * amp * prev;
        // }

        prev = n;
        freq *= lacunarity;
        amp *= gain;
    }

    return sum;
}

float pattern(in vec2 p, out vec2 q, out vec2 r) {
    p *= scale1;
    q = vec2(fbm(p + offsetA), fbm(p + offsetB));
    r = vec2(fbm(p + scale2 * q + offsetC), fbm(p + scale2 * q + offsetD));

    return fbm(p + scale2 * r);
}

void main() {
    vec2 q;
    vec2 r;
    float f = pattern(isf_FragNormCoord * 2.0 - 1.0, q, r);

    // vec3 color =
    //     mix(mix(mix(color1.rgb, color2.rgb, f), color3.rgb, length(q) * 0.5),
    //         color4.rgb, r.y * 0.5);
    float d = smoothstep(line_thickness, 0.0, fract(f * noise_scale));

    vec3 color = mix(color1.rgb, color2.rgb, d);

    gl_FragColor = vec4(color, 1);
}
