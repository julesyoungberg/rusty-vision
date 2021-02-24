#version 450

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 frag_color;

layout(set = 0, binding = 0) uniform GeneralUniforms {
    vec2 mouse;
    vec2 resolution;
    float time;
};

layout(set = 1, binding = 0) uniform sampler spectrum_sampler;
layout(set = 1, binding = 1) uniform texture2D spectrum;

#define PI 3.14159265359
#define ITERATIONS 10

// based on https://www.shadertoy.com/view/llGcRK
void main() {
    vec2 st = uv * resolution / resolution.y;
    st *= 0.5;
    // st *= cos(time * 0.5) + 1.5;

    float angle = time * 0.3;
    float c = cos(angle);
    float s = sin(angle);
    st *= mat2(c, -s, s, c);

    vec3 color = vec3(0.0);

    float scale = PI / 3;
    float m = 0.5;

    for (float i = 0.0; i < ITERATIONS; i += 1.0) {
        float angle = time * i * 0.01 + length(st) * PI * (sin(time * 0.05) * 0.3 - 0.4);
        float c = cos(angle);
        float s = sin(angle);
        st *= mat2(c, -s, s, c);

        float theta = atan(st.x, st.y) + PI;
        theta = (floor(theta / scale) + 0.5) * scale;

        vec2 dir = vec2(sin(theta), cos(theta));
        vec2 codir = dir.yx * vec2(-1, 1);

        st = vec2(dot(dir, st), dot(codir, st));
        st += vec2(time * 0.1, time * 0.15) * 0.035 * i;
        // st = abs(fract(st + 0.5) * 2.0 - 1.0) * 0.7;
        st = fract(st);

        float spec1 = texture(sampler2D(spectrum, spectrum_sampler), vec2(((i * 3)) / (ITERATIONS * 3), 0)).x;
        float spec2 = texture(sampler2D(spectrum, spectrum_sampler), vec2(((i * 3) + 1) / (ITERATIONS * 3), 0)).x;
        float spec3 = texture(sampler2D(spectrum, spectrum_sampler), vec2(((i * 3) + 2) / (ITERATIONS * 3), 0)).x;
        vec3 p = vec3(spec1, spec2, spec3) * exp(i) * 0.001 * vec3(1, 5, 9);
        color += exp(-min(st.x, st.y) * 16.0) * (cos(p * i + time * 0.5) * 0.5 + 0.5) * m;
        m *= 0.9;
    }

    color.rgb *= 1.2;

	frag_color = vec4(color, 1.0);
}
