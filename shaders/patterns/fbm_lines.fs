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

float noise_hash3(vec3 p) {
    p = fract(p * 0.3183099 + .1);
    p *= 17.0;
    return fract(p.x * p.y * p.z * (p.x + p.y + p.z));
}

float noise31(in vec3 x) {
    vec3 i = floor(x);
    vec3 f = fract(x);
    f = f * f * (3.0 - 2.0 * f);

    return mix(mix(mix(noise_hash3(i + vec3(0, 0, 0)),
                       noise_hash3(i + vec3(1, 0, 0)), f.x),
                   mix(noise_hash3(i + vec3(0, 1, 0)),
                       noise_hash3(i + vec3(1, 1, 0)), f.x),
                   f.y),
               mix(mix(noise_hash3(i + vec3(0, 0, 1)),
                       noise_hash3(i + vec3(1, 0, 1)), f.x),
                   mix(noise_hash3(i + vec3(0, 1, 1)),
                       noise_hash3(i + vec3(1, 1, 1)), f.x),
                   f.y),
               f.z);
}

float fbm(in vec2 p) {
    const mat2 m = mat2(0.8, 0.6, -0.6, 0.8);
    float f = 0.0;
    float scale = 0.5;
    float scaling = 0.0;

    for (float i = 0.0; i < octaves; i++) {
        f += scale * noise31(vec3(p, TIME * speed));
        p *= m;
        scaling += scale;
        scale *= 0.5;
    }

    f /= scaling;
    return f;
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
