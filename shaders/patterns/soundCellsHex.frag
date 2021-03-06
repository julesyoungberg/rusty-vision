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

#define TAU 6.28318530718

//@import util/hsv2rgb
//@import util/noise
//@import util/rand

vec3 hsv2rgb(vec3 c);
float noise2(in vec2 p);
vec2 rand2(vec2 p);

const vec2 s = vec2(1, 1.7320508);

// shane's hexagonal tiling (https://www.shadertoy.com/view/llSyDh)
vec4 get_hex(vec2 p) {
    vec4 hc = floor(vec4(p, p - vec2(0.5, 1)) / s.xyxy) + 0.5;
    vec4 h = vec4(p - hc.xy * s, p - (hc.zw + 0.5) * s);
    return (dot(h.xy, h.xy) < dot(h.zw, h.zw)) ? vec4(h.xy, hc.xy) : vec4(h.zw, hc.zw + vec2(0.5, 1));
}

float hex_dist(in vec2 p) {
    p = abs(p);
    return max(dot(p, normalize(s)), p.x);
}

vec2 get_point(vec2 coord) {
    vec2 point = rand2(coord);
    return vec2(
        cos(time * 0.5 + point.x * TAU),
        sin(time * 0.5 + point.y * TAU)
    ) * 0.3;
}

vec3 voronoi(vec4 coords, vec2 st, float scale) {
    vec2 gv = coords.xy;
    vec2 id = coords.zw;

    float m_dist = scale;
    vec2 m_point;
    vec2 m_coord;
    vec2 m_diff;

    // find the nearest cell center
    #pragma unroll
    for (float i = 0.0; i <= 6.0; i++) {
        vec2 offset = vec2(0);
        if (i < 6.0) {
            float angle = i * TAU / 6.0;
            float si = sin(angle);
            float co = cos(angle);
            offset = vec2(1.0, 0.0) * mat2(co, -si, si, co);
        }

        vec2 coord = get_hex(st + offset).zw;
        vec2 point = get_point(coord);

        vec2 diff = offset + point - gv;
        float dist = length(diff);

        if (dist < m_dist) {
            m_dist = dist;
            m_point = point;
            m_coord = coord;
            m_diff = diff;
        }
    }

    float m_edge_dist = scale;

    //find the nearest edge
    #pragma unroll
    for (float i = 0.0; i <= 6.0; i++) {
        vec2 offset = vec2(0);
        if (i < 6.0) {
            float angle = i * TAU / 6.0;
            float si = sin(angle);
            float co = cos(angle);
            offset = vec2(1.0, 0.0) * mat2(co, -si, si, co);
        }

        vec2 coord = get_hex(st + offset).zw;
        if (all(equal(m_coord, coord))) {
            continue;
        }

        vec2 point = get_point(coord);

        vec2 diff = offset + point - gv;
        float dist = length(diff);

        vec2 to_center = (m_diff + diff) * 0.5;
        vec2 cell_diff = normalize(diff - m_diff);
        float edge_dist = dot(to_center, cell_diff);
        m_edge_dist = min(m_edge_dist, edge_dist);
    }

    return vec3(m_point, m_edge_dist);
}

//  Function from Iñigo Quiles
// https://www.iquilezles.org/www/articles/functions/functions.htm
float impulse(float x, float k) {
    float h = k * x;
    return h * exp(1.0 - h);
}

void main() {
    vec2 st = uv;
    st.y *= resolution.y / resolution.x;

    vec3 color = vec3(0);

    float r = length(st);
    float scaling = 1.0;
    if (r < 0.5) {
        scaling = pow(smoothstep(0.0, 0.5, r) * 2.0, 4.0);
    } else {
        scaling = 16.0 - smoothstep(0.5, 0.75, r) * 2.0;
    }
    st *= scaling;
    st += time;

    float scale = 1.0;
    st *= scale;

    vec4 coords = get_hex(st);
    vec3 val = voronoi(coords, st, scale);
    vec2 m_point = val.xy;
    float m_edge_dist = val.z;

    // color = hsv2rgb(vec3(fract(dot(m_point, m_point)), 1, 1));
    // color = mix(vec3(0), color, smoothstep(0.01, 0.02, m_edge_dist));
    // // color += scaling;
    // float radius = 1.0;
    // color += smoothstep(radius, radius + 0.01, r) - smoothstep(radius + 0.01, radius + 0.02, r);

    // map point to 1d value between 0 and 1
    float point_val = fract(dot(m_point, m_point) * 4.38);
    float intensity = texture(sampler2D(spectrum, spectrum_sampler), vec2(point_val, 0)).x;

    color = hsv2rgb(vec3(point_val, 1, 1)).zxy * log(intensity * 10.0);
    color = mix(vec3(0), color, smoothstep(0.05, 0.06, m_edge_dist));

    // dots
    // color += 1.0 - step(0.02, m_dist);
    // grid
    // color.r += step(0.48, hex_dist(gv));

    frag_color = vec4(color, 1);
}
