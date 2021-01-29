#version 450

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 frag_color;

layout(set = 0, binding = 0) uniform AudioUniforms {
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

void main() {
    vec3 tristimulus = vec3(tristimulus1, tristimulus2, tristimulus3);
    frag_color = vec4(tristimulus, 1);
}
