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

#define BANDS 16

float circle(in vec2 st, in float radius) { 
    vec2 dist = st - vec2(0.5);
	return 1.0 - smoothstep(
        radius - (radius * 0.01), 
        radius + (radius * 0.01), 
        dot(dist, dist) * 4.0
    );
}

void main() {
    vec2 st = uv;
    st.y *= resolution.y / resolution.x;
    st = st * 0.5 + 0.5;
    st.x *= BANDS;

    vec2 tilePos = floor(st);
    st = fract(st);

    vec3 tristimulus = vec3(tristimulus1, tristimulus2, tristimulus3);
    float bandLoudness = texture(sampler2D(spectrum, audio_sampler), vec2(tilePos.x / BANDS, 0)).x;
    vec3 color = tristimulus + vec3(circle(st, bandLoudness * 0.05));
    // vec3 color = tristimulus + vec3(circle(st, clamp(log(bandLoudness + 1.0), 0.0, 0.25)));

    // rectangle bands
    // float scaling = 15.0 * tilePos.x;
    // vec3 color = mix(vec3(0), tristimulus, step(bandLoudness * scaling, st.y));

    frag_color = vec4(color, 1);
}
