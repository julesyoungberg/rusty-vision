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

//@import util/complex_inv
//@import util/complex_mult
//@import util/palette

vec2 complex_inv(in vec2 z);
vec2 complex_mult(in vec2 a, in vec2 b);
vec3 palette(in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d);

// based on Circuits by Kali
// https://www.shadertoy.com/view/XlX3Rj
vec3 formula(in vec2 st, in vec2 c) {
    vec2 z = st;
    float last_stable = 0.0;

    // orbit traps
    float min_comp = 1000.0;
    float min_mag = min_comp;

    float angle = time * 0.05;

    const float iterations = 9;
    for (float i = 0.0; i < iterations; i++) {
        z *= mat2(cos(angle), -sin(angle), sin(angle), cos(angle));
        z = abs(complex_inv(z)) + c;

        float mag = length(z);

        // orbit traps
        float w = 0.1;
        // get minimum component
        float m_comp = clamp(abs(min(z.x, z.y)), w - mag, abs(mag - w));
        // update overall minimum component
        min_comp = min(m_comp, min_comp);
        // update minimum magnitude
        min_mag = min(mag, min_mag);
        // m is 0 unless minimum == min_comp
        // catches the lasst i where z is stable
        last_stable = max(last_stable, i * (1.0 - abs(sign(min_comp - m_comp))));
    }

    last_stable += 1.0;

    float intensity = 0.01;
    float width = intensity * last_stable * 2.0;

    float circ = pow(max(0.0, width - min_mag * 0.1) / width, 6.0);
    float shape = max(pow(max(0.0, width - min_comp) / width, 0.25), circ);

    float t = time * 0.1;
    vec3 color = palette(
        last_stable / iterations,
        vec3(0.5, 0.5, 0.5), 
        vec3(0.5, 0.5, 0.5),
        vec3(1.0, 1.0, 1.0),
        fract(vec3(
            texture(sampler2D(spectrum, spectrum_sampler), vec2(0.7, 0)).x,
            texture(sampler2D(spectrum, spectrum_sampler), vec2(0.4, 0)).x + 0.1,
            texture(sampler2D(spectrum, spectrum_sampler), vec2(0.1, 0)).x + 0.2
        ))
    );

    // carve out the pattern
    color *= 0.4 + mod(last_stable / iterations + min_mag * 0.2 - t, 1.0) * 1.6;

    return color * shape;
}

void main() {
    vec2 st = uv;
    st.x *= resolution.x / resolution.y;
    st *= 10.0;
    
    vec3 color = vec3(0);

    float t = time;
    float scale = 1.5 + sin(t / 25.0);
    st *= scale;

    vec2 c = vec2(-0.6);
    c += vec2(sin(t / 11.0), sin(t / 13.0)) * 0.5;

    color = formula(st, c);

    frag_color = vec4(color, 1);
}
