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

vec3 palette(in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d);

// based on Illustrated Equations by sben
// https://www.shadertoy.com/view/MtBGDW0
void main() {
    vec2 st = uv;
    st.x *= resolution.x / resolution.y;

    vec3 color = vec3(0.0);

    float strength = texture(sampler2D(spectrum, spectrum_sampler), vec2(0.04, 0)).x;

    st *= 10.0;
    st *= mix(1.0, 0.95, strength);

    vec2 p = abs(st * 2.0);
    vec2 ab = vec2(2.0 - p.x);
    float t = time;

    for (float i = 0.0; i < 12.0; i++) {
        ab += p + cos(length(p) - t) * sin(t * 0.1);
        p.y += sin(ab.x - p.x - t) * 0.5;
        p.x += sin(ab.y - t) * 0.5;
        p -= p.x + p.y;
        p += (st.y + cos(st.x) * sin(t * 0.267)) * sin(t * 0.345);
        ab += vec2(p.y);
    }

    p /= 30.0;

    float id = p.x * 2.0 + p.y;

    color = palette(
        id,
        vec3(0.5, 0.5, 0.5), 
        vec3(0.5, 0.5, 0.5),
        vec3(1.0, 1.0, 1.0),
        fract(vec3(
                texture(sampler2D(spectrum, spectrum_sampler), vec2(0.7, 0)).x,
                texture(sampler2D(spectrum, spectrum_sampler), vec2(0.4, 0)).x,
                texture(sampler2D(spectrum, spectrum_sampler), vec2(0.1, 0)).x
        ))
    );

    float size = mix(0.1, 0.8, strength);
    color = mix(color, vec3(0.0), smoothstep(size, size + 0.01, id));
    color *= vec3(0.5, 1.0, 1.2);

    frag_color = vec4(sqrt(color), 1);
}
