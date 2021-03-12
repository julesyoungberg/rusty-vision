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

//@import util/hsv2rgb
//@import util/rand

vec3 hsv2rgb(vec3 c);
vec2 rand2(vec2 p);

vec2 get_point(vec2 coord) {
    vec2 point = rand2(coord);
    point = sin(6.2831 * point) * 0.5 + 0.5;
    return point;
}

vec4 voronoi(vec2 p, float scale) {
    vec2 i_st = floor(p);
    vec2 f_st = fract(p) - 0.5;

    float m_dist = scale;
    vec2 m_point;
    vec2 m_coord;
    vec2 m_diff;

    // find the nearest cell center
    #pragma unroll
    for (int y = -1; y <= 1; y++) {
        #pragma unroll
        for (int x = -1; x <= 1; x++) {
            vec2 neighbor = vec2(x, y);
            vec2 coord = i_st + neighbor;
            vec2 point = get_point(coord);

            vec2 diff = neighbor + point - f_st;
            float dist = length(diff);

            if (dist < m_dist) {
                m_dist = dist;
                m_point = point;
                m_coord = coord;
                m_diff = diff;
            }
        }
    }

    float m_edge_dist = scale;

    // find the nearest edge
    #pragma unroll
    for (int y = -1; y <= 1; y++) {
        #pragma unroll
        for (int x = -1; x <= 1; x++) {
            vec2 neighbor = vec2(x, y);
            vec2 coord = i_st + neighbor;
            if (all(equal(m_coord, coord))) {
                continue;
            }

            vec2 point = get_point(coord);

            vec2 diff = neighbor + point - f_st;
            float dist = length(diff);

            vec2 to_center = (m_diff + diff) * 0.5;
            vec2 cell_diff = normalize(diff - m_diff);
            float edge_dist = dot(to_center, cell_diff);
            m_edge_dist = min(m_edge_dist, edge_dist);
        }
    }

    return vec4(m_coord + m_point, m_dist, m_edge_dist);
}

void main() {
    vec2 st = uv;
    st.y *= resolution.y / resolution.x;

    vec3 color = vec3(0.0);

    // Scale
    float scale = 2.0;
    st *= scale;

    vec4 val = voronoi(st, scale);
    vec2 m_point = val.xy;
    float m_dist = val.z;
    float m_edge_dist = val.w;

    vec2 gv = fract(st) - 0.5;
    vec2 id = floor(st);
    vec2 relative_point = m_point - id;
    vec2 cell_uv = gv - relative_point;

    // color.rg = cell_uv;

    scale = 2.0;
    vec4 inner_val = voronoi(cell_uv * scale, scale);
    vec2 inner_point = inner_val.xy;
    float inner_dist = inner_val.z;
    float inner_edge_dist = inner_val.w;

    color += inner_edge_dist;

    // map point to 1d value between 0 and 1
    // float point_val = dot(m_point, m_point) * 0.5;
    // float intensity = texture(sampler2D(spectrum, spectrum_sampler), vec2(point_val, 0)).x;

    // vec3 color = hsv2rgb(vec3(point_val, 1, 1)).zxy * log(intensity * 10.0);
    // color = mix(vec3(0), color, smoothstep(0.05, 0.06, m_edge_dist));

    color += smoothstep(0.02, 0.0, m_edge_dist);

    // Draw cell center
    color += 1.-step(.02, m_dist);

    // Draw grid
    color.r += step(.48, gv.x) + step(.48, gv.y);

    frag_color = vec4(color, 1.0);
}
