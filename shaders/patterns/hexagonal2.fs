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
            "NAME": "audio_reactive",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 1.0,
            "DEFAULT": 1.0
        },
        {
            "NAME": "audio_sensitivity",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 5.0,
            "DEFAULT": 3.0
        },
        {
            "NAME": "scale",
            "TYPE": "float",
            "MIN": 1.0,
            "MAX": 20.0,
            "DEFAULT": 5.0
        },
        {
            "NAME": "h_speed",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 1.0,
            "DEFAULT": 0.2
        },
        {
            "NAME": "s_speed",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 1.0,
            "DEFAULT": 0.13
        },
        {
            "NAME": "v_speed",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 1.0,
            "DEFAULT": 0.11
        }
    ]
}*/

vec3 hsv2rgb(vec3 c) {
    vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
    return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}

bool above_line(vec2 r, vec2 q, vec2 p) {
    return dot(vec2(q.y - r.y, r.x - q.x), q - p) > 0.0;
}

float hex_dist(in vec2 p) {
    p = abs(p);
    return max(dot(p, normalize(vec2(1.0, sqrt(3)))), p.x);
}

vec4 hex_coords(in vec2 st) {
    vec2 r = vec2(1, sqrt(3));
    vec2 h = r * 0.5;

    vec2 a = mod(st, r) - h;
    vec2 b = mod(st - h, r) - h;

    vec2 gv = length(a) < length(b) ? a : b;

    vec2 id = st - gv;

    return vec4(gv, id);
}

void main() {
    vec2 st = isf_FragNormCoord * 2.0 - 1.0;
    st.y *= RENDERSIZE.y / RENDERSIZE.x;

    vec3 color = vec3(0);

    st *= scale;

    vec4 coords = hex_coords(st);
    vec2 gv = coords.xy;
    vec2 id = coords.zw;

    float i = dot(id, id + vec2(13, 17));

    vec2 center = vec2(0, 0);
    vec2 right = vec2(sqrt(3.0) * 0.5, 0.5);
    vec2 left = vec2(-sqrt(3.0) * 0.5, 0.5);
    vec2 bottom = vec2(0.0, -1.0);

    bool cr = above_line(center, right, gv);
    bool cl = above_line(center, left, gv);
    bool cb = above_line(center, bottom, gv);

    float hue = 0.0;
    if (cr && !cl) { // top
        hue = 0.0;
    } else if (!cr && cb) { // right
        hue = 0.33;
    } else if (cl && !cb) { // left
        hue = 0.66;
    }

    float h = hue + 0.5;
    color = hsv2rgb(
        vec3(mod(hue + TIME * h_speed * h + i * 0.4, 1),
             0.6 + sin(TIME * s_speed * h + i * 0.3 + hue * 2.23) * 0.3,
             0.7 + sin(TIME * v_speed * h + i * 0.7 + hue * 3.55) * 0.2));

    if (audio_reactive > 0.5) {
        float intensity =
            log(IMG_NORM_PIXEL(
                    fft_texture,
                    vec2(fract(dot(id, id) * 0.1 + hue + TIME * 0.01), 0.0))
                        .x *
                    audio_sensitivity +
                1.0);
        color *= clamp(log(intensity * 2.0), 0.3, 1.1);
    }

    gl_FragColor = vec4(color, 1);
}
