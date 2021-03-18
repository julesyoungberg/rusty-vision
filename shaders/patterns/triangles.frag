#version 450

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 frag_color;

layout(set = 0, binding = 0) uniform GeneralUniforms {
    vec2 mouse;
    vec2 resolution;
    float time;
    int mouse_down;
};

layout(set = 1, binding = 0) uniform sampler spectrum_sampler;
layout(set = 1, binding = 1) uniform texture2D spectrum;

#define AUDIO_REACTIVE 1
#define TAU 6.28318530718

const vec2 s = vec2(1, 1.7320508);

//@import util/above_line
//@import util/line_dist
//@import util/power_curve
//@import util/pulse
//@import util/rand

bool above_line(vec2 r, vec2 q, vec2 p);
float line_dist(vec2 p, vec2 a, vec2 b);
float power_curve(float x, float a, float b);
float pulse(float c, float w, float x);
float rand(float n);
float rand21(vec2 co);
vec2 rand2(vec2 p);

// shane's hexagonal tiling (https://www.shadertoy.com/view/llSyDh)
vec4 get_hex(vec2 p) {
    vec4 hc = floor(vec4(p, p - vec2(0.5, 1)) / s.xyxy) + 0.5;
    vec4 h = vec4(p - hc.xy * s, p - (hc.zw + 0.5) * s);
    return (dot(h.xy, h.xy) < dot(h.zw, h.zw)) ? vec4(h.xy, hc.xy) : vec4(h.zw, hc.zw + vec2(0.5, 1));
}

vec2 get_point(vec2 id) {
    return sin(rand2(id) * time) * 0.15;
}

float line(vec2 p, vec2 a, vec2 b) {
    float d = line_dist(p, a, b);
    float m = smoothstep(0.02, 0.0, d);
    return m;
}

void main() {
    vec2 st = uv;
    st.y *= resolution.y / resolution.x;
    st = st * 0.5 + 0.5;

    vec3 color = vec3(0);

    const float scale = 17.0;
    st *= scale;

    vec4 hex_coords = get_hex(st);
    vec2 gv = hex_coords.xy;
    vec2 id = hex_coords.zw;

    // color.rg = id * 0.05;

    vec2 points[7];
    vec2 coords[7];

    // collect neighboring points
    for (int i = 0; i <= 6; i++) {
        vec2 offset = vec2(0);
        if (i < 6.0) {
            float angle = float(i) * TAU / 6.0;
            float si = sin(angle);
            float co = cos(angle);
            offset = vec2(1.0, 0.0) * mat2(co, -si, si, co);
        }

        vec2 coord = get_hex(st + offset).zw;
        coords[i] = coord;
        points[i] = get_point(coord) + offset;
    }

    // find the current triangle
    bool l1 = above_line(points[6], points[0], gv);
    bool l2 = above_line(points[6], points[1], gv);
    bool l3 = above_line(points[6], points[2], gv);
    bool l4 = above_line(points[6], points[3], gv);
    bool l5 = above_line(points[6], points[4], gv);
    bool l6 = above_line(points[6], points[5], gv);

    int n1 = 0;
    int n2 = 0;

    // get neighboring coords for the current triangle
    if (l1 && !l2) {
        // top right
        n1 = 0;
        n2 = 1;
    } else if (l2 && !l3) {
        // top
        n1 = 1;
        n2 = 2;
    } else if (l3 && !l4) {
        // top left
        n1 = 2;
        n2 = 3;
    } else if (l4 && !l5) {
        // bottom left
        n1 = 3;
        n2 = 4;
    } else if (l5 && !l6) {
        // bottom
        n1 = 4;
        n2 = 5;
    } else if (l6 && !l1) {
        // bottom right
        n1 = 5;
        n2 = 0;
    }

    vec2 c1 = coords[n1];
    vec2 c2 = coords[n2];
    vec2 tri_coord = (id + c1 + c2) / 3.0;
    tri_coord /= scale;

    // color gradient
    float d1 = 1.0 - length(tri_coord - vec2(0.33, 0.5)) * 2.0;
    color = mix(vec3(0.0), vec3(0.19, 0.25, 0.43), d1 * step(0.01, d1));
    float d2 = 1.0 - length(tri_coord - vec2(0.85, 0.25)) * 2.0;
    color = mix(color, vec3(0.35, 0.06, 0.28), d2 * step(0.01, d2));

    // shimmer
    float dist = length(tri_coord) * 4.0;
    float t = dist - time * 6.0 + (tri_coord.x + tri_coord.y) * 2.0;
    float shine = mix(1.0, 2.5, pulse(3.0, 2.0, mod(t, 17.5)));
    color *= shine;

    // randomly darkened tiles
    float darkness = rand21(tri_coord) * 0.5 + 0.5;
    color *= darkness;

    // randomly sparkling tiles
    vec2 rnd = rand2(tri_coord);
    float ti = rand(dot(rnd, rnd) * 0.1);
    float sparkle = 1.0;
    if (AUDIO_REACTIVE == 1) {
        float intensity = texture(sampler2D(spectrum, spectrum_sampler), vec2(fract(ti), 0)).x;
        sparkle = intensity + 1.0;
    } else {
        float loop = 30.0;
        float t2 = time * 0.5 + ti * loop;
        sparkle = mix(1.0, 5.0, max(0.0, power_curve(mod(t2, loop), 2.0, 1.0)));
    }
    color *= sparkle;

    // draw lines
    for (int i = 0; i < 6; i++) {
        color += line(gv, points[6], points[i]) * 0.03;
    }

    // correct center point
    float correction = length(gv - points[6]);
    color -= smoothstep(0.02, 0.0, correction) * 0.1;

    frag_color = vec4(pow(color, vec3(1.5)), 1);
}
