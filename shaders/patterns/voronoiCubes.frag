#version 450

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 frag_color;

layout(set = 0, binding = 0) uniform GeneralUniforms {
    vec2 mouse;
    vec2 resolution;
    float time;
    int mouse_down;
};

// based on VoronoiCubes  by mojovideotech
// https://editor.isf.video/shaders/5e7a7ff97c113618206de819

#define C30 0.866025 // cos 30
#define TAU 6.28318530718

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

    float m_corner_dist = 8.0;
    float m_corner_dist2 = 0.0;
    float m_id = 0.0;
    float m_side = 0.0;

    #pragma unroll
    for (float y = -2.0; y <= 2.0; y++) {
        #pragma unroll
        for (float x = -2.0; x <= 2.0; x++) {
            vec2 offset = vec2(x, y);
            vec2 coord = id + offset;
            vec2 point = get_point(coord);
            vec2 diff = offset + point - gv;

            // regular voronoi distance calc
            vec2 d1 = vec2(length(diff), 1.0);
            // voronoi cube distance calc
            vec2 d2 = vec2(
                max(abs(diff.x) * C30 + diff.y * 0.5, -diff.y),
                step(0.0, 0.5 * abs(diff.x) + C30 * diff.y) * (1.0 + step(0.0, diff.x))
            );
            // blend the two modes
            vec2 d = mix(d2, d1, fract(mode));

            // update minimums
            if (d.x < m_corner_dist) {
                m_corner_dist2 = m_corner_dist;
                m_corner_dist = d.x;
                m_id = fract(length(coord));
                m_side = d.y;
            } else if (d.x < m_corner_dist2) {
                m_corner_dist2 = d.x;
            }
        }
    }

    return vec4(m_corner_dist, m_id, m_side * 0.5, m_corner_dist2 - m_corner_dist);
}

void main() {
    vec2 st = uv;
    st.y *= resolution.y / resolution.x;
    st = st * 0.5 + 0.5;

    float mode = 0.0; // sin(time) * 0.5 + 0.5;

    float scale = 12.0;
    vec4 val = voronoi(st * scale, mode);
    
    // unique cell color
    vec3 color = sin(val.y * 10.0 + vec3(2, 1, 0.5) + time) * 0.5 + 0.5;
    // slide edge shading
    color *= sqrt(clamp(1.0 - val.x, 0.0, 1.0));
    // cube face shading
    color *= clamp((1.0 - val.z) * 0.5 + 0.5, 0.0, 1.0);

    frag_color = vec4(color, 1.0);
}
