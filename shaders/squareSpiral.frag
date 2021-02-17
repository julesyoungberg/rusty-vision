#version 450

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 frag_color;

layout(set = 0, binding = 0) uniform GeneralUniforms {
    vec2 mouse;
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

#define PI 3.14159265359
#define ITERATIONS 32.0

//@import util/hsv2rgb

vec3 hsv2rgb(vec3 c);

float sdBox(in vec2 p, in vec2 b) {
    vec2 d = abs(p) - b;
    return length(max(d, 0.0)) + min(max(d.x, d.y), 0.0);
}

float square(in vec2 p) {
    float dist = sdBox(p, vec2(1));
    return smoothstep(0.005, 0.0, dist) - smoothstep(0.0, -0.005, dist);
}

void main() {
    vec2 st = uv;
    st.y *= resolution.y / resolution.x;
    st *= 10.0;

    float angle = time;
    float c = cos(angle);
    float s = sin(angle);
    st *= mat2(c, -s, s, c);
    
    vec3 color = vec3(0);

    for (float i = 0.0; i < ITERATIONS; i += 1.0) {
        float f = i / ITERATIONS;
        float intensity = texture(sampler2D(spectrum, audio_sampler), vec2(f, 0)).x;
        color += square(st) * hsv2rgb(vec3(f, 1, 1)) * intensity; // * exp(i * 0.5);

        angle = (i + 1) * PI * 0.002 * sin(time * 0.5);
        c = cos(angle);
        s = sin(angle);
        st *= mat2(c, -s, s, c);
        st *= sin(time * 0.7) * 0.05 + 0.95;
    }

    frag_color = vec4(color, 1.0);
}
