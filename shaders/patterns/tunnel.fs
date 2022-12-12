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
                0.60,
                0.10,
                0.20,
                1.0
            ]
        },
        {
            "NAME": "sensitivity",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 1.0,
            "DEFAULT": 0.5
        },
        {
            "NAME": "brightness",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 1.0,
            "DEFAULT": 0.5
        },
        {
            "NAME": "camera_speed",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 0.5,
            "DEFAULT": 0.1
        },
        {
            "NAME": "rotation_speed",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 0.5,
            "DEFAULT": 0.1
        },
        {
            "NAME": "color_speed",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 1.0,
            "DEFAULT": 0.1
        },
        {
            "NAME": "n_layers",
            "TYPE": "float",
            "MIN": 4.0,
            "MAX": 100.0,
            "DEFAULT": 32.0
        },
        {
            "NAME": "color_amount",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 1.0,
            "DEFAULT": 1.0
        }
    ]
}*/

#define PI 3.14159265359

float rand(float n) { return fract(n * 1183.5437 + .42); }

float circle(vec2 st, float size) {
    return smoothstep(size + 0.01, size, length(st));
}

float angle_slice(vec2 st, float start, float end) {
    float a = atan(st.y, st.x) / PI;
    a = a * 0.5 + 0.5;

    if (end < start) {
        end += 1.0;
        if (a <= end) {
            a += 1.0;
        }
    }

    return step(start, a) - step(end, a);
}

float semi_circle(vec2 st, float r1, float r2, float a1, float a2) {
    float d = circle(st, r2) - circle(st, r1);
    return d * angle_slice(st, a1, a2);
}

// IQ's palette generator:
// https://www.iquilezles.org/www/articles/palettes/palettes.htm
vec3 palette(in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d) {
    return a + b * cos(6.28318 * (c * t + d));
}

float get_strength(float i) {
    return mix(
        brightness,
        log(IMG_NORM_PIXEL(fft_texture, vec2(i, 0)).x + 1.0),
        sensitivity
    );
}

// draws 1 layer of the pseudo-3d effect
vec3 layer(in vec2 st, float n) {
    float layer_rotation = TIME * rotation_speed * rand(n);

    float s = sin(layer_rotation);
    float c = cos(layer_rotation);
    mat2 rot = mat2(c, -s, s, c);
    st *= rot;

    vec3 color = vec3(0);

    float r1 = 0.2 + rand(n) * 0.6;
    float r2 = r1 + rand(n + 3.0 * n) * 0.01 + 0.05;
    float a1 = rand(n + 13.0 * n);
    float a2 = fract(a1 + rand(n + 17.0 * n) * 0.5 + 0.25);

    color += semi_circle(st, r1, r2, a1, a2);

    vec3 clr = palette(
        fract(n * 4.0 + TIME * color_speed),
        vec3(0.5, 0.5, 0.5),
        vec3(0.5, 0.5, 0.5),
        vec3(1.0, 1.0, 1.0),
        color_config.rgb
    );

    clr = mix(vec3(1.0), clr, color_amount);

    return color * clr * get_strength(fract(n * 2.0));
}

void main() {
    vec2 st = isf_FragNormCoord - 0.5;
    st.x *= RENDERSIZE.x / RENDERSIZE.y;

    vec3 color = vec3(0.0);

    float gradient = st.y * camera_speed;

    float t = TIME * camera_speed;
    float rt = TIME * rotation_speed;
    // vec3 base = sin(t * 5.0 * vec3(0.345, 0.456, 0.657)) * 0.4 + 0.6;

    float s = sin(rt);
    float c = cos(rt);
    mat2 rot = mat2(c, -s, s, c);
    st *= rot;

    float rot_shift = 0.5;
    s = sin(rot_shift);
    c = cos(rot_shift);
    rot = mat2(c, -s, s, c);

    float stp = 1.0 / n_layers;

    for (float i = 0.0; i < 1.0; i += stp) {
        float z = fract(i * 2.0 + t);
        float size = mix(10.0, 0.5, z);
        float fade = smoothstep(0.0, 0.5, z) * smoothstep(1.0, 0.9, z);
        st *= rot;
        color += layer(st * size, i) * fade;
    }

    // float gradient_strength = texture(sampler2D(spectrum, spectrum_sampler),
    // vec2(0.1, 0)).x; color -= gradient * base * gradient_strength;

    gl_FragColor = vec4(color, 1);
}
