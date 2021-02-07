#version 450

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 frag_color;

layout(set = 0, binding = 0) uniform GeneralUniforms {
    vec2 resolution;
    float time;
};

layout(set = 1, binding = 0) uniform sampler audioSampler;
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
    float spectralCentroid;
    float spectralComplexity;
    float spectralContrast;
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

void main() {
    vec2 st = uv;
    st.y *= resolution.y / resolution.x;
    st = st * 0.5 + 0.5;

    // Scale
    st *= 20.0;

    // Tile the space
    vec2 i_st = floor(st);
    vec2 f_st = fract(st);

    float m_dist = 20.0;
    vec2 m_point;

    for (int y = -1; y <= 1; y++) {
        for (int x = -1; x <= 1; x++) {
            vec2 neighbor = vec2(x, y);
            vec2 point = rand2(i_st + neighbor);

            point = sin(time + 6.2831 * point) * 0.5 + 0.5;

            vec2 diff = neighbor + point - f_st;
            float dist = length(diff);

            if (dist < m_dist) {
                m_dist = dist;
                m_point = point;
            }
        }
    }

    // map point to 1d value between 0 and 1
    float point_val = dot(m_point, m_point) * 0.5;
    float intensity = texture(sampler2D(mfccs, audioSampler), vec2(point_val, 0)).x;

    vec3 color = hsv2rgb(vec3(point_val, 1, 1)).zxy * log(intensity * 200.0);

    frag_color = vec4(color, 1.0);
}
