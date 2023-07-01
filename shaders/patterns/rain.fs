/*{
    "DESCRIPTION": "Rain",
    "CREDIT": "by julesyoungberg",
    "ISFVSN": "2.0",
    "CATEGORIES": [ "GENERATOR" ],
    "INPUTS": [
        {
            "NAME": "drop_color",
            "TYPE": "color",
            "DEFAULT": [
                1.0,
                1.0,
                1.0,
                1.0
            ]
        },
        {
            "NAME": "scale",
            "TYPE": "float",
            "MIN": 1.0,
            "MAX": 10.0,
            "DEFAULT": 2.0
        },
        {
            "NAME": "rotation",
            "TYPE": "float",
            "MIN": -1.0,
            "MAX": 1.0,
            "DEFAULT": 0.0
        },
        {
            "NAME": "density",
            "TYPE": "float",
            "MIN": 1.0,
            "MAX": 100.0,
            "DEFAULT": 10.0
        },
        {
            "NAME": "drop_height",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 0.5,
            "DEFAULT": 0.0
        },
        {
            "NAME": "drop_width",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 0.1,
            "DEFAULT": 0.1
        },
        {
            "NAME": "speed",
            "TYPE": "float",
            "MIN": 0.1,
            "MAX": 5.0,
            "DEFAULT": 3.0
        },
        {
            "NAME": "sway_speed",
            "TYPE": "float",
            "MIN": 0.01,
            "MAX": 0.2,
            "DEFAULT": 0.1
        }
    ]
}*/

float noise_hash2(vec2 p) {
    p = 50.0 * fract(p * 0.3183099 + vec2(0.71, 0.113));
    return -1.0 + 2.0 * fract(p.x * p.y * (p.x + p.y));
}

float noise_hash3(vec3 p) {
    p = fract(p * 0.3183099 + .1);
    p *= 17.0;
    return fract(p.x * p.y * p.z * (p.x + p.y + p.z));
}

float noise21(in vec2 p) {
    vec2 i = floor(p);
    vec2 f = fract(p);
    vec2 u = f * f * (3.0 - 2.0 * f);

    return mix(mix(noise_hash2(i + vec2(0.0, 0.0)),
                   noise_hash2(i + vec2(1.0, 0.0)), u.x),
               mix(noise_hash2(i + vec2(0.0, 1.0)),
                   noise_hash2(i + vec2(1.0, 1.0)), u.x),
               u.y);
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
    f += 0.500 * noise21(p);
    p *= m * 2.02;
    f += 0.250 * noise21(p);
    p *= m * 2.03;
    f += 0.125 * noise21(p);
    p *= m * 2.01;
    f += 0.0625 * noise21(p);
    p *= m * 2.04;
    f /= 0.9375;
    return f * 0.5 + 0.5;
}

void main() {
    vec2 st = isf_FragNormCoord;
    vec2 grid_scale = scale * vec2(4.0, 1.0);
    st *= grid_scale;
    st *= mat2(cos(rotation), -sin(rotation), sin(rotation), cos(rotation));
    st.y = mod(st.y, grid_scale.y);
    vec2 id = floor(st);
    vec2 gv = fract(st);

    vec3 color = vec3(0.0);

    for (float i = -1.0; i <= 1.0; i += 1.0) {
        float column = id.x + i;
        float c = column / grid_scale.x;

        for (float j = 0.0; j < 1.0; j += 1.0 / density) {
            float id = fbm(vec2(j, c));
            float height = mix(
                0.1 / grid_scale.y,
                (0.1 + drop_height) / grid_scale.y,
                noise21(vec2(c, j) * 0.5)
            );

            float width = mix(
                0.02,
                0.02 + drop_width,
                noise21(vec2(j, c) * id)
            );

            float y_pos = mod(
                TIME * -speed + noise_hash2(vec2(j, c) * id * 0.5) * grid_scale.y,
                grid_scale.y
            );

            float x_pos = noise31(
                vec3(c, j, TIME * sway_speed + noise_hash2(vec2(c, j) * id * 0.1))
            ) * 2.0 + column - 1.0;

            color += (
                (step(y_pos - height * 0.5, st.y) - step(y_pos + height * 0.5, st.y))
                * (step(x_pos, st.x) - step(x_pos + width, st.x))
            );
        }
    }

    gl_FragColor = vec4(color * drop_color.rgb, 1);
}
