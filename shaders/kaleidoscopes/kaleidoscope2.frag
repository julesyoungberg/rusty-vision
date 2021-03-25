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

// based on [SH17A] Fractal Thingy #2 by Klems
// https://www.shadertoy.com/view/Xd2Bzw
void main() {
    vec2 st = uv;
    st.y *= resolution.y / resolution.x;
    st *= 0.5;
    
    vec3 color = vec3(0.0);

    // breathing effect
    st += st * sin(dot(st, st) * 20.0 - time) * 0.04;

    const float iterations = 8.0;
    for (float i = 0.5; i < iterations; i++) {
        // fractal formula
        st = abs(2.0 * fract(st - 0.5) - 1.0);
        
        // rotation
        st *= mat2(cos(time * 0.01 * i * i + 0.78 * vec4(1, 7, 3, 1)));

        float spec_strength = texture(sampler2D(spectrum, spectrum_sampler), vec2(i / iterations, 0.0)).x;
        float strength = clamp(spec_strength, 0.0, 1.0) * i;
        color += exp(-abs(st.y) * 5.0) * (cos(vec3(1.0, 3.0, 2.0) * i + time * 0.1) * 0.5 + 0.5) * strength;
    }

    color *= 0.5;
    // color.rg *= 0.5;

    frag_color = vec4(color, 1);
}
