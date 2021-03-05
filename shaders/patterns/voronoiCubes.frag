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

// based on: https://editor.isf.video/shaders/5e7a7ff97c113618206de819

#define C30 0.866025 // cos 30
#define TAO 6.28318530718

//@import util/hsv2rgb
//@import util/rand

vec3 hsv2rgb(vec3 c);
float rand(float n);
vec2 rand2(vec2 p);

vec2 get_point(vec2 coord) {
    vec2 point = rand2(coord);
    point = sin(time * 0.5 + 6.2831 * point) * 0.5 + 0.5;
    return point;
}

vec4 voronoi(in vec2 p, float mode) {
    vec2 gv = fract(p);
    vec2 id = floor(p);

    vec3 m = vec3(8.0);
    float m_dist = 0.0;

    for (float y = -2.0; y <= 2.0; y++) {
        for (float x = -2.0; x <= 2.0; x++) {
            vec2 offset = vec2(x, y);
            vec2 coord = id + offset;
            vec2 point = get_point(coord);

            vec2 diff = offset + point - gv;
            vec2 d1 = vec2(sqrt(dot(diff, diff)), 1.0);
            vec2 d2 = vec2(
                max(abs(diff.x) * C30 + diff.y * 0.5, -diff.y),
                step(0.0, 0.5 * abs(diff.x) + C30 * diff.y) * (1.0 + step(0.0, diff.x))
            );
            vec2 d = mix(d2, d1, fract(mode));

            if (d.x < m.x) {
                m_dist = m.x;
                m.x = d.x;
                m.y = rand(dot(id + offset, vec2(1)));
                m.z = d.y;
            } else if (d.x < m_dist) {
                m_dist = d.x;
            }
        }
    }

    return vec4(m, m_dist - m.x);
}

void main() {
    vec2 st = uv;
    st.y *= resolution.y / resolution.x;
    st = st * 0.5 + 0.5;

    float mode = mod(time * 0.2, 3.0);
    mode = floor(mode) + smoothstep(0.8, 1.0, fract(mode));

    float scale = 10.0;
    vec4 val = voronoi((24.0 - scale) * st, mode);
    
    vec3 color = sin(val.y * 2.5 + vec3(2, 1, 0.5));
    color *= sqrt(clamp(1.0 - val.x, 0.0, 1.0));
    color *= clamp(0.5 + (1.0 - val.z / 2.0) * 0.5, 0.0, 1.0);

    frag_color = vec4(color, 1.0);
}
