#version 450

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 frag_color;

layout(set = 0, binding = 0) uniform GeneralUniforms {
    vec2 mouse;
    vec2 resolution;
    float time;
    int mouse_down;
};

//@import util/rand

vec2 rand2(vec2 p);

// from the Art of Code

float line_dist(vec2 p, vec2 a, vec2 b) {
    vec2 pa = p - a;
    vec2 ba = b - a;
    float t = clamp(dot(pa, ba) / dot(ba, ba), 0.0, 1.0);
    return length(pa - ba * t);
}

float line(vec2 p, vec2 a, vec2 b) {
    float d = line_dist(p, a, b);
    float m = smoothstep(0.03, 0.01, d);
    m *= smoothstep(1.2, 0.8, length(a - b));
    return m;
}

vec2 get_point(vec2 id) {
    return sin(rand2(id) * time) * 0.4;
}

void main() {
    vec2 st = uv;
    st.x *= resolution.x / resolution.y;

    vec3 color = vec3(0.0);

    st *= 3.0;
    vec2 gv = fract(st) - 0.5;
    vec2 id = floor(st);

    vec2 points[9];
    int i = 0;

    for (float y = -1.0; y <= 1.0; y++) {
        for (float x = -1.0; x <= 1.0; x++) {
            points[i++] = get_point(id + vec2(x, y)) + vec2(x, y);
        }
    }

    float m = 0.0;
    float t = time * 2.0;
    for (int j = 0; j < 9; j++) {
        m += line(gv, points[4], points[j]);

        vec2 q = (points[j] - gv) * 25.0;
        float sparkle = 1.0 / (dot(q, q) * 2.0);
        m += sparkle * (sin(t + points[j].x * 10.0) * 0.5 + 0.5);
    }

    m += line(gv, points[1], points[3]);
    m += line(gv, points[1], points[5]);
    m += line(gv, points[7], points[3]);
    m += line(gv, points[7], points[5]);

    color += m;

    frag_color = vec4(color, 1);
}
