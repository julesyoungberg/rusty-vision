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

#define BANDS 16
#define LIGHTS 5

//@import util/hsv2rgb

vec3 hsv2rgb(vec3 c);

const vec2 lights[LIGHTS] = vec2[LIGHTS](
    vec2(0.5), 
    vec2(0.4, 0.6), 
    vec2(0.6, 0.4), 
    vec2(0.4, 0.4),
    vec2(0.6, 0.6)
);

void main() {
    vec2 st = uv;
    st.y *= resolution.y / resolution.x;
    st = st * 0.5 + 0.5;

    vec3 color = vec3(0);

    #pragma unroll 1
    for (int i = 0; i < LIGHTS; i++) {
        float f = float(i) / float(LIGHTS);
        float intensity = texture(sampler2D(spectrum, audio_sampler), vec2(f, 0.0)).x;

        float brightness = log(intensity * exp(i) * 0.001 + 1.0);

        vec2 light_pos = lights[i];
        light_pos.x += cos(time * (float(i + 1) / float(LIGHTS)) * 0.5 + f * 3.14) * 0.1;
        light_pos.y += sin(time * (float(i + 1) / float(LIGHTS)) * 0.5 + f * 3.14) * 0.1;

        vec3 light_color = hsv2rgb(vec3(f + time * 0.1, 1, 1));

        color += brightness / distance(st, light_pos) * light_color;
    }

    frag_color = vec4(color, 1);
}
