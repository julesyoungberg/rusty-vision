#version 450

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 frag_color;

layout(set = 0, binding = 0) uniform GeneralUniforms {
    vec2 resolution;
    float time;
};

layout(set = 1, binding = 0) uniform AudioUniforms {
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

float circle(in vec2 st, in float radius){
    vec2 dist = st - vec2(0.0);
	return 1.0 - smoothstep(radius - (radius * 0.01), radius + (radius * 0.01), dot(dist, dist) * 4.0);
}

void main() {
    vec2 st = uv;
    st.x *= resolution.x / resolution.y;

    vec3 tristimulus = vec3(tristimulus1, tristimulus2, tristimulus3);
    vec3 color = tristimulus + vec3(circle(st, loudness * 20.0));

    frag_color = vec4(color, 1);
}
