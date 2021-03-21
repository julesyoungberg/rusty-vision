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

#define SPECTRUM_SIZE 32

//@import util/palette
//@import util/rand

vec3 palette(in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d);
vec2 rand2(vec2 p);
float rand21(vec2 p);

// based on Illustrated Equations by sben
// https://www.shadertoy.com/view/MtBGDW0
// and Circuits by Kali
// https://www.shadertoy.com/view/XlX3Rj
void main() {
    vec2 st = uv;
    st.x *= resolution.x / resolution.y;

    vec3 color = vec3(0.0);

    float strength = texture(sampler2D(spectrum, spectrum_sampler), vec2(0.04, 0)).x;

    st *= 2.0;
    st *= mix(1.0, 0.95, strength);

    vec2 p = abs(st * 2.0);
    vec2 ab = vec2(2.0 - p.x);
    float t = time;

    // orbit traps
    float min_comp = 1000.0;
    float min_mag = min_comp;
    float last_stable = 0.0;
    const float iterations = 12.0;

    for (float i = 0.0; i < iterations; i++) {
        // fractal equation
        ab += p + cos(length(p) - t) * sin(t * 0.1);
        p.y += sin(ab.x - p.x - t) * 0.5;
        p.x += sin(ab.y - t) * 0.5;
        p -= p.x + p.y;
        p += (st.y + cos(st.x) * sin(t * 0.267)) * sin(t * 0.345);
        ab += vec2(p.y);

        // update orbit traps
        float mag = length(p);
        float w = 0.1;
        float m_comp = clamp(abs(min(p.x, p.y)), w - mag, abs(mag - w));
        min_comp = min(m_comp, min_comp);
        min_mag = min(mag * 0.1, min_mag);
        last_stable = max(last_stable, i * (1.0 - abs(sign(min_comp - m_comp))));
    }

    p /= 30.0;

    float id = p.x * 2.0 + p.y;

    // get fractal color
    color = palette(
        id * 2.0,
        vec3(0.5, 0.5, 0.5), 
        vec3(0.5, 0.5, 0.5),
        vec3(1.0, 1.0, 1.0),
        fract(vec3(
                texture(sampler2D(spectrum, spectrum_sampler), vec2(0.7, 0)).x,
                texture(sampler2D(spectrum, spectrum_sampler), vec2(0.4, 0)).x,
                texture(sampler2D(spectrum, spectrum_sampler), vec2(0.1, 0)).x
        ))
    );

    // carve out design
    last_stable += 1.0;
    float intensity = 0.01;
    float width = intensity * last_stable * 2.0;
    float circ = pow(max(0.0, width - min_mag) / width, 6.0);
    float shape = max(pow(max(0.0, width - min_comp) / width, 0.25), circ);
    color *= shape;

    frag_color = vec4(sqrt(color), 1);
}
