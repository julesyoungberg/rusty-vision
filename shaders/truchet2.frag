#version 450

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 frag_color;

layout(set = 0, binding = 0) uniform GeneralUniforms {
    vec2 mouse;
    vec2 resolution;
    float time;
};

#define PI 3.14159265359

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

float hash21(vec2 p) {
    p = fract(p * vec2(234.34, 435.345));
    p += dot(p, p);
    return fract(p.x * p.y);
}

bool is_above_line(vec2 r, vec2 q, vec2 p) {
    return dot(vec2(q.y - r.y, r.x - q.x), q - p) > 0.0;
}

void main() {
    vec2 st = uv;
    st.y *= resolution.y / resolution.x;

    vec3 color = vec3(0);

    // st += time * 0.05;
    st *= 6.0;

    vec4 coords = hex_coords(st);
    vec2 gv = coords.xy;
    vec2 id = coords.zw;
    float edge_dist = 0.5 - hex_dist(gv);

    float h = hash21(id + 0.001);
    if (h < 0.5) {
        gv.y *= -1.0;
    }

    vec2 center = vec2(0, 0);
    vec2 right = vec2(sqrt(3.0) * 0.5, 0.5);
    vec2 left = vec2(-sqrt(3.0) * 0.5, 0.5);
    vec2 bottom = vec2(0.0, -1.0);

    bool cr = is_above_line(center, right, gv);
    bool cl = is_above_line(center, left, gv);
    bool cb = is_above_line(center, bottom, gv);

    if (!cr && cl) { // bottom
        vec2 n = normalize(vec2(1, sqrt(3)));
        gv -= n * min(dot(gv, n), 0.0) * 2.0;
    } else if (cr && !cb) { // left
        gv.x = abs(gv.x);
    }

    vec2 c_uv = gv - vec2(0.5, 1 / (2 * sqrt(3)));
    float d = abs(length(c_uv) - 0.289);
    float sd = floor(mod(d * 25.0 - time, 3));
    // color += sd / 3.0;

    vec3 color1 = vec3(0.95, 0.32, 0.06);
    vec3 color2 = vec3(0.87, 0.71, 0.28);
    vec3 color3 = vec3(0.03, 0.26, 0.34);

    color = mix(mix(color1, color2, sd), color3, sd - 1.0);

    float width = 0.1;
    float mask = smoothstep(0.01, -0.01, d - width);

    color = mix(color, color * smoothstep(0.1, 0.0, d), mask);

    // TODO figure out UV coords for inside the truchet pattern
    // refrence: https://www.shadertoy.com/view/llSyDh
    float angle = atan(gv.x - 0.5, gv.y - 1 / (2 * sqrt(3)));
    color.g += angle * mask;
    // float checker = mod(id.x + id.y + 0.001, 2.0) * 2.0 - 1.0;

    // float x = fract(angle / 2.09);
    // float y = (d - (0.5 - width)) / (width * 2.0);
    // color += mask; // * x;

    // if (edge_dist < 0.01) color = vec3(1, 0, 0);

    frag_color = vec4(color, 1);
}
