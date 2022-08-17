/*{
    "DESCRIPTION": "Grid colored by noise",
    "CREDIT": "by julesyoungberg",
    "ISFVSN": "2.0",
    "CATEGORIES": [ "GENERATOR" ],
    "INPUTS": [
        {
            "NAME": "noise_octaves",
            "TYPE": "float",
            "MIN": 1.0,
            "MAX": 10.0,
            "DEFAULT": 5.0
        },
        {
            "NAME": "noise_speed",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 2.0,
            "DEFAULT": 0.3
        },
        {
            "NAME": "noise_scale_x",
            "TYPE": "float",
            "MIN": 1.0,
            "MAX": 20.0,
            "DEFAULT": 6.0
        },
        {
            "NAME": "noise_scale_y",
            "TYPE": "float",
            "MIN": 1.0,
            "MAX": 20.0,
            "DEFAULT": 6.0
        },
        {
            "NAME": "grid_size",
            "TYPE": "float",
            "MIN": 0.1,
            "MAX": 100.0,
            "DEFAULT": 50.0
        },
        {
            "NAME": "grid_scale",
            "TYPE": "float",
            "MIN": 1.0,
            "MAX": 100.0,
            "DEFAULT": 1.0
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

    for (float i = 0.0; i < noise_octaves; i++) {
        f += scale * noise31(vec3(p, TIME * noise_speed + scale));
        p *= m;
        scaling += scale;
        scale *= 0.5;
    }

    f /= scaling;
    return f;
}

void main() {
    vec2 st = isf_FragNormCoord;
    float ratio = RENDERSIZE.x / RENDERSIZE.y;
    st.x *= ratio;

    st = mix(
        st,
        floor(st * grid_size) / grid_scale,
        min(1.0, grid_size)
    );

    vec3 color = vec3(0.0);

    color += smoothstep(0.4, 0.5, fbm(st * vec2(noise_scale_x, noise_scale_y)));

    gl_FragColor = vec4(color, 1);
}
