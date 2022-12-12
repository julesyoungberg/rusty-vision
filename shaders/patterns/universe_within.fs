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
            "NAME": "scale",
            "TYPE": "float",
            "MIN": 0.5,
            "MAX": 2.0,
            "DEFAULT": 1.0
        },
        {
            "NAME": "line_thickenss",
            "TYPE": "float",
            "MIN": 0.01,
            "MAX": 0.05,
            "DEFAULT": 0.02
        },
        {
            "NAME": "camera_speed",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 0.5,
            "DEFAULT": 0.1
        },
        {
            "NAME": "point_speed",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 2.0,
            "DEFAULT": 1.0
        },
        {
            "NAME": "color_speed",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 2.0,
            "DEFAULT": 1.0
        }
    ]
}*/

// based on The Universe Within by BigWings
// https://www.shadertoy.com/view/lscczl
// from the Art of Code

vec3 hsv2rgb(vec3 c) {
    vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
    return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}

float line_dist(vec2 p, vec2 a, vec2 b) {
    vec2 pa = p - a;
    vec2 ba = b - a;
    float t = clamp(dot(pa, ba) / dot(ba, ba), 0.0, 1.0);
    return length(pa - ba * t);
}

float rand(float n) { return fract(n * 1183.5437 + .42); }

float rand21(vec2 p) {
    return fract(sin(dot(p.xy, vec2(12.9898, 78.233))) * 43758.5453);
}

vec2 rand2(vec2 p) {
    return fract(
        sin(vec2(dot(p, vec2(127.1, 311.7)), dot(p, vec2(269.5, 183.3)))) *
        43758.5453);
}

float line(vec2 p, vec2 a, vec2 b, float strength) {
    float d = line_dist(p, a, b);
    float s = mix(0.0, line_thickenss, strength);
    float m = smoothstep(s, s - 0.01, d);
    float d2 = length(a - b);
    m *= smoothstep(1.2, 0.8, d2) + smoothstep(0.05, 0.03, d2 - 0.75);
    return m;
}

vec2 get_point(vec2 id) { return sin(rand2(id) * TIME * point_speed) * 0.4; }

float get_strength(float i) {
    return mix(
        brightness,
        log(IMG_NORM_PIXEL(fft_texture, vec2(i, 0)).x + 1.0),
        sensitivity
    );
}

// draws 1 layer of the pseudo-3d effect
vec3 layer(vec2 st, float n) {
    vec2 gv = fract(st) - 0.5;
    vec2 id = floor(st) + n;

    vec2 points[9];
    float ids[9];
    float strengths[9];
    vec3 colors[9];
    float t = TIME * color_speed;
    int i = 0;

    vec3 color = vec3(0);

    // collect neighboring points
    for (float y = -1.0; y <= 1.0; y++) {
        for (float x = -1.0; x <= 1.0; x++) {
            vec2 coord = id + vec2(x, y);
            points[i] = get_point(coord) + vec2(x, y);
            ids[i] = rand21(coord);
            strengths[i] = get_strength(ids[i]);
            colors[i] = hsv2rgb(vec3(fract(ids[i] + t * 0.1), 1, 1)) + 0.01;
            i++;
        }
    }

    float line_strength;
    vec3 line_color;
    float line_brightness = 0.5;

    // draw points and lines
    for (int j = 0; j < 9; j++) {
        line_strength = (strengths[4] + strengths[j]) / 2.0;
        line_color = (colors[4] + colors[j]) / 2.0 * line_brightness;
        color += line(gv, points[4], points[j], line_strength) * line_color;

        float d = length(gv - points[j]);
        float sparkle = 0.003 / (d * d);
        sparkle *= smoothstep(1.0, 0.7, d);
        color += sparkle * colors[j] * strengths[j];
    }

    // draw lines that pass through the center without a point in it
    line_strength = (strengths[1] + strengths[3]) / 2.0;
    line_color = (colors[1] + colors[3]) / 2.0 * line_brightness;
    color += line(gv, points[1], points[3], line_strength) * line_color;

    line_strength = (strengths[1] + strengths[5]) / 2.0;
    line_color = (colors[1] + colors[5]) / 2.0 * line_brightness;
    color += line(gv, points[1], points[5], line_strength) * line_color;

    line_strength = (strengths[7] + strengths[3]) / 2.0;
    line_color = (colors[7] + colors[3]) / 2.0 * line_brightness;
    color += line(gv, points[7], points[3], line_strength) * line_color;

    line_strength = (strengths[7] + strengths[3]) / 2.0;
    line_color = (colors[7] + colors[7]) / 2.0 * line_brightness;
    color += line(gv, points[7], points[5], line_strength) * line_color;

    return color;
}

void main() {
    vec2 st = isf_FragNormCoord - 0.5;
    st.x *= RENDERSIZE.x / RENDERSIZE.y;
    st *= scale;

    vec3 color = vec3(0.0);

    float gradient = st.y * camera_speed;

    float t = TIME * camera_speed;
    // vec3 base = sin(t * 5.0 * vec3(0.345, 0.456, 0.657)) * 0.4 + 0.6;

    float s = sin(t);
    float c = cos(t);
    mat2 rot = mat2(c, -s, s, c);
    st *= rot;

    float rot_shift = 0.5;
    s = sin(rot_shift);
    c = cos(rot_shift);
    rot = mat2(c, -s, s, c);

    for (float i = 0.0; i < 1.0; i += 0.25) {
        float z = fract(i + t);
        float size = mix(7.0, 0.5, z);
        float fade = smoothstep(0.0, 0.5, z) * smoothstep(1.0, 0.8, z);
        st *= rot;
        color += layer(st * size + i * vec2(20.0, 27.0), i) * fade;
    }

    // float gradient_strength = texture(sampler2D(spectrum, spectrum_sampler),
    // vec2(0.1, 0)).x; color -= gradient * base * gradient_strength;

    gl_FragColor = vec4(color, 1);
}
