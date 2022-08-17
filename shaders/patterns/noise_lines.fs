/*{
    "DESCRIPTION": "Lines displaced by noise",
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
            "MAX": 0.6,
            "DEFAULT": 0.3
        },
        {
            "NAME": "shift_speed",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 0.6,
            "DEFAULT": 0.3
        },
        {
            "NAME": "num_lines",
            "TYPE": "float",
            "MIN": 1.0,
            "MAX": 60.0,
            "DEFAULT": 20.0
        },
        {
            "NAME": "line_spacing",
            "TYPE": "float",
            "MIN": 0.01,
            "MAX": 0.1,
            "DEFAULT": 0.05
        },
        {
            "NAME": "noise_width",
            "TYPE": "float",
            "MIN": 0.1,
            "MAX": 1.0,
            "DEFAULT": 0.9
        },
        {
            "NAME": "noise_amount",
            "TYPE": "float",
            "MIN": 0.1,
            "MAX": 0.6,
            "DEFAULT": 0.3
        },
        {
            "NAME": "noise_scale_x",
            "TYPE": "float",
            "MIN": 1.0,
            "MAX": 7.0,
            "DEFAULT": 2.0
        },
        {
            "NAME": "noise_scale_y",
            "TYPE": "float",
            "MIN": 1.0,
            "MAX": 7.0,
            "DEFAULT": 4.0
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
        f += scale * noise31(vec3(p, TIME * noise_speed));
        p *= m;
        scaling += scale;
        scale *= 0.5;
    }

    f /= scaling;
    return f;
}

float line_dist(vec2 p, vec2 a, vec2 b) {
    vec2 pa = p - a;
    vec2 ba = b - a;
    float t = clamp(dot(pa, ba) / dot(ba, ba), 0.0, 1.0);
    return length(pa - ba * t);
}

float line(vec2 p, vec2 a, vec2 b) {
    float d = line_dist(p, a, b);
    float m = smoothstep(0.002, 0.0, d);
    return m;
}

float displaced_line(in vec2 st, float line_y, float x_len) {
    float radius = x_len * noise_width * 0.5;
    float center = x_len * 0.5;
    float start = center - radius;
    float end = center + radius;
    float shift = TIME * shift_speed;
    float shifted_x = mod(st.x - shift, x_len);
    float amount = smoothstep(start, center, shifted_x) - smoothstep(center, end, shifted_x);
    st.y -= noise_amount * fbm(st * vec2(noise_scale_x, noise_scale_y)) * amount;
    return line(st, vec2(0.0, line_y), vec2(x_len, line_y));
}

void main() {
    vec2 st = isf_FragNormCoord;
    float ratio = RENDERSIZE.x / RENDERSIZE.y;
    st.x *= ratio;

    vec3 color = vec3(0.0);

    float lines_width = (num_lines - 1.0) * line_spacing;
    float start = 0.5 - lines_width * 0.5;
    float end = 0.5 + lines_width * 0.5;

    for (float y = start; y <= end; y += line_spacing) {
        color += displaced_line(st, y, ratio);
    }

    gl_FragColor = vec4(color, 1);
}
