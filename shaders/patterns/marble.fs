#version 450
/*{
    "DESCRIPTION": "FBM marble pattern generator based on the Book of Shaders",
    "CREDIT": "by julesyoungberg",
    "ISFVSN": "2.0",
    "CATEGORIES": [ "GENERATOR" ],
    "INPUTS": [
        {
            "NAME": "gain",
            "TYPE": "float"
        },
        {
            "NAME": "lacunarity",
            "TYPE": "float"
        },
        {
            "NAME": "scale1",
            "TYPE": "float"
        },
        {
            "NAME": "scale2",
            "TYPE": "float"
        },
        {
            "NAME": "noise_mode",
            "TYPE": "float",
            "MAX": 2.0
        },
        {
            "NAME": "octaves",
            "TYPE": "float",
            "MIN": 1.0,
            "MAX": 4.0
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
            "NAME": "time_scale",
            "TYPE": "point2D"
        },
        {
            "NAME": "color1",
            "TYPE": "color"
        },
        {
            "NAME": "color2",
            "TYPE": "color"
        },
        {
            "NAME": "color3",
            "TYPE": "color"
        },
        {
            "NAME": "color4",
            "TYPE": "color"
        }
    ]
}*/

// Some useful functions
vec3 mod289(vec3 x) { return x - floor(x * (1.0 / 289.0)) * 289.0; }
vec2 mod289(vec2 x) { return x - floor(x * (1.0 / 289.0)) * 289.0; }
vec3 permute(vec3 x) { return mod289(((x * 34.0) + 1.0) * x); }

//
// Description : GLSL 2D simplex noise function
//      Author : Ian McEwan, Ashima Arts
//  Maintainer : ijm
//     Lastmod : 20110822 (ijm)
//     License :
//  Copyright (C) 2011 Ashima Arts. All rights reserved.
//  Distributed under the MIT License. See LICENSE file.
//  https://github.com/ashima/webgl-noise
//
float snoise(vec2 v) {
    // Precompute values for skewed triangular grid
    const vec4 C = vec4(0.211324865405187,
                        // (3.0-sqrt(3.0))/6.0
                        0.366025403784439,
                        // 0.5*(sqrt(3.0)-1.0)
                        -0.577350269189626,
                        // -1.0 + 2.0 * C.x
                        0.024390243902439);
    // 1.0 / 41.0

    // First corner (x0)
    vec2 i = floor(v + dot(v, C.yy));
    vec2 x0 = v - i + dot(i, C.xx);

    // Other two corners (x1, x2)
    vec2 i1 = vec2(0.0);
    i1 = (x0.x > x0.y) ? vec2(1.0, 0.0) : vec2(0.0, 1.0);
    vec2 x1 = x0.xy + C.xx - i1;
    vec2 x2 = x0.xy + C.zz;

    // Do some permutations to avoid
    // truncation effects in permutation
    i = mod289(i);
    vec3 p = permute(permute(i.y + vec3(0.0, i1.y, 1.0)) + i.x +
                     vec3(0.0, i1.x, 1.0));

    vec3 m = max(0.5 - vec3(dot(x0, x0), dot(x1, x1), dot(x2, x2)), 0.0);

    m = m * m;
    m = m * m;

    // Gradients:
    //  41 pts uniformly over a line, mapped onto a diamond
    //  The ring size 17*17 = 289 is close to a multiple
    //      of 41 (41*7 = 287)

    vec3 x = 2.0 * fract(p * C.www) - 1.0;
    vec3 h = abs(x) - 0.5;
    vec3 ox = floor(x + 0.5);
    vec3 a0 = x - ox;

    // Normalise gradients implicitly by scaling m
    // Approximation of: m *= inversesqrt(a0*a0 + h*h);
    m *= 1.79284291400159 - 0.85373472095314 * (a0 * a0 + h * h);

    // Compute final noise value at P
    vec3 g = vec3(0.0);
    g.x = a0.x * x0.x + h.x * x0.y;
    g.yz = a0.yz * vec2(x1.x, x2.x) + h.yz * vec2(x1.y, x2.y);
    return 130.0 * dot(m, g);
}

float getNoiseVal(vec2 p) {
    float raw = snoise(p);

    if (noise_mode == 1.0) {
        return abs(raw);
    }

    return raw * 0.5 + 0.5;
}

float fbm(vec2 p) {
    float sum = 0.0;
    float freq = 1.0;
    float amp = 0.5;
    float prev = 1.0;

    for (int i = 0; i < octaves; i++) {
        float n = getNoiseVal(p * freq);

        // if (invert) {
        //     n = 1.0 - n;
        // }

        // if (sharpen) {
        //     n = n * n;
        // }

        sum += n * amp;

        // if (scaleByPrev) {
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
    p += offset;

    float t = 0.0;
    t = TIME * 0.1;

    q = vec2(fbm(p + offsetA + t * time_scale.x),
             fbm(p + offsetB - t * time_scale.y));
    r = vec2(fbm(p + scale2 * q + offsetC), fbm(p + scale2 * q + offsetD));

    return fbm(p + scale2 * r);
}

void main() {
    vec4 finalColor = vec4(0.0);

    vec2 q;
    vec2 r;
    float f = pattern(isf_FragNormCoord, q, r);

    finalColor = mix(color1, color2, f);
    finalColor = mix(finalColor, color3, length(q) / 2.0);
    finalColor = mix(finalColor, color4, r.y / 2.0);

    gl_FragColor = finalColor;
}
