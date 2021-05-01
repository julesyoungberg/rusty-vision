/*{
    "DESCRIPTION": "FBM marble pattern generator based on the Book of Shaders",
    "CREDIT": "by julesyoungberg",
    "ISFVSN": "2.0",
    "CATEGORIES": [ "GENERATOR" ],
    "INPUTS": [
        {
            "NAME": "gain",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 1.0,
            "DEFAULT": 0.5
        },
        {
            "NAME": "lacunarity",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 3.0,
            "DEFAULT": 2.0
        },
        {
            "NAME": "scale1",
            "TYPE": "float",
            "MIN": 1.0,
            "MAX": 4.0,
            "DEFAULT": 3.0
        },
        {
            "NAME": "scale2",
            "TYPE": "float",
            "MIN": 1.0,
            "MAX": 4.0,
            "DEFAULT": 3.0
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
            "DEFAULT": 5.0
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
            "TYPE": "point2D",
            "MIN": [0.0, 0.0],
            "MAX": [1.0, 1.0],
            "DEFAULT": [0.1, 0.1]
        },
        {
            "NAME": "color1",
            "TYPE": "color",
            "DEFAULT": [0.5, 0.1, 0.1, 1.0]
        },
        {
            "NAME": "color2",
            "TYPE": "color",
            "DEFAULT": [1.0, 1.0, 1.0, 1.0]
        },
        {
            "NAME": "color3",
            "TYPE": "color",
            "DEFAULT": [0.43, 0.43, 0.43, 1.0]
        },
        {
            "NAME": "color4",
            "TYPE": "color",
            "DEFAULT": [1.0, 1.0, 1.0, 1.0]
        }
    ]
}*/

float noise_hash2(vec2 p) {
    p = 50.0 * fract(p * 0.3183099 + vec2(0.71, 0.113));
    return -1.0 + 2.0 * fract(p.x * p.y * (p.x + p.y));
}

float noise(in vec2 p) {
    vec2 i = floor(p);
    vec2 f = fract(p);
    vec2 u = f * f * (3.0 - 2.0 * f);

    return mix(mix(noise_hash2(i + vec2(0.0, 0.0)),
                   noise_hash2(i + vec2(1.0, 0.0)), u.x),
               mix(noise_hash2(i + vec2(0.0, 1.0)),
                   noise_hash2(i + vec2(1.0, 1.0)), u.x),
               u.y);
}

float getNoiseVal(vec2 p) {
    float raw = noise(p);

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
        n = 1.0 - n;
        // }

        // if (sharpen) {
        n = n * n;
        // }

        sum += n * amp;

        // if (scaleByPrev) {
        // sum += n * amp * prev;
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
