#version 450

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 frag_color;

layout(set = 0, binding = 0) uniform GeneralUniforms {
    vec2 mouse;
    vec2 resolution;
    float time;
    int mouse_down;
};

layout(set = 1, binding = 0) uniform sampler webcam_sampler;
layout(set = 1, binding = 1) uniform texture2D webcam;
layout(set = 1, binding = 2) uniform WebcamUniforms {
    vec2 video_size;
};

//@import util/rand
//@import util/hsv2rgb

vec3 rand3(vec3 p);
vec3 hsv2rgb(vec3 c);

vec3 webcam_color(in vec2 coord) {
    vec2 c = vec2(coord.x, 1.0 - coord.y);
    return texture(sampler2D(webcam, webcam_sampler), fract(c)).rgb;
}

vec3 get_point(vec3 coord) {
    vec3 point = rand3(coord);
    point = sin(time * 0.2 + 6.2831 * point) * 0.5 + 0.5;
    return point;
}

vec4 voroni(vec3 p, float scale) {
    vec3 i_st = floor(p);
    vec3 f_st = fract(p);

    float m_dist = scale;
    vec3 m_point;
    vec3 m_coord;
    vec3 m_diff;

    // find the nearest cell center
    #pragma unroll
    for (int z = -1; z <= 1; z++) {
        #pragma unroll
        for (int y = -1; y <= 1; y++) {
            #pragma unroll
            for (int x = -1; x <= 1; x++) {
                vec3 neighbor = vec3(x, y, z);
                vec3 coord = i_st + neighbor;
                vec3 point = get_point(coord);

                vec3 diff = neighbor + point - f_st;
                float dist = length(diff);

                if (dist < m_dist) {
                    m_dist = dist;
                    m_point = point;
                    m_coord = coord;
                    m_diff = diff;
                }
            }
        }
    }

    return vec4(m_coord + m_point, m_dist);
}

void main() {
    vec2 st = uv;
    st = st * 0.5 + 0.5;

    float scale = 20.0;
    st *= scale;

    vec3 p = vec3(st, time * 0.4);
    vec4 val = voroni(p, scale);
    vec3 m_point = val.xyz;
    float m_dist = val.w;
    
    vec2 g_point = m_point.xy;
    vec2 coord = g_point / scale;
    vec3 color = webcam_color(coord);
    // color = mix(vec3(0), color, smoothstep(0.01, 0.02, m_edge_dist));
    color *= (1.0 - m_dist) * 1.1;
    
	frag_color = vec4(color, 1.0);
}
