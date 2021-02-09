#version 450

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 frag_color;

layout(set = 0, binding = 0) uniform GeneralUniforms {
    vec2 resolution;
    float time;
};

layout(set = 1, binding = 0) uniform sampler audio_sampler;
layout(set = 1, binding = 1) uniform texture2D mfccs;
layout(set = 1, binding = 2) uniform texture2D spectrum;
layout(set = 1, binding = 3) uniform AudioUniforms {
    float dissonance;
    float energy;
    float loudness;
    float noisiness;
    float onset;
    float pitch;
    float rms;
    float spectral_centroid;
    float spectral_complexity;
    float spectral_contrast;
    float tristimulus1;
    float tristimulus2;
    float tristimulus3;
};

#define NUM_MFCCS 12

//@import util/rand
//@import util/hsv2rgb

vec2 rand2(vec2 p);
vec3 hsv2rgb(vec3 c);

//https://www.shadertoy.com/view/4djSRW
float hash(vec2 p) {
	vec3 p3  = fract(vec3(p.xyx) * .1031);
    p3 += dot(p3, p3.yzx + 19.19);
    return fract((p3.x + p3.y) * p3.z);
}

vec2 get_point(vec2 coord) {
    vec2 point = rand2(coord);
    point = sin(time + 6.2831 * point) * 0.5 + 0.5;
    return point;
}

void main() {
    vec2 st = uv;
    st.y *= resolution.y / resolution.x;
    st = st * 0.5 + 0.5;

    // Scale
    float scale = 20.0;
    st *= scale;

    // Tile the space
    vec2 i_st = floor(st);
    vec2 f_st = fract(st);

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

    // map point to 1d value between 0 and 1
    float point_val = dot(m_point, m_point) * 0.5;
    float intensity = texture(sampler2D(spectrum, audio_sampler), vec2(point_val, 0)).x;

    vec3 color = hsv2rgb(vec3(point_val, 1, 1)).zxy * log(intensity * 10.0);
    color = mix(vec3(0), color, step(0.1, m_edge_dist));

    // Draw cell center
    // color += 1.-step(.02, m_dist);

    // Draw grid
    // color.r += step(.98, f_st.x) + step(.98, f_st.y);

    frag_color = vec4(color, 1.0);
}
