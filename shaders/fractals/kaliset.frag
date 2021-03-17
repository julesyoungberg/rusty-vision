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

//@import util/palette
//@import util/tri_wave

vec3 palette(in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d);
float tri_wave(float x);

// based on Simplicity by JoshP
// https://www.shadertoy.com/view/lslGWr
vec4 kaliset(in vec3 z) {
    float strength = 8.0;// - sin(time * 0.1);
    float power = 2.5;// + sin(time * 0.15) * 0.5;
	float accum = 0.0;
	float prev = 0.0;
	float tw = 0.0;
    float t = time * 0.1;

    for (float i = 0.0; i < 32.0; i++) {
        z = abs(z);
        float mag = dot(z, z);
        z /= mag;
        z += vec3(
            -0.6 + sin(t) * 0.3,
            -0.4 + sin(t * 2.7 + 2.1) * 0.3,
            -1.5 + sin(t * 0.7 + 1.3) * 0.01
        );

        float w = exp(-i / 7.0);
        accum += w * exp(-strength * pow(abs(mag - prev), power));
        tw += w;
    }

    return vec4(z, max(0.0, 5.0 * accum / tw - 0.7));
}

void main() {
    vec2 st = uv;
    st.x *= resolution.x / resolution.y;
    
    vec3 p = vec3(st, (tri_wave(time * 0.003) - 0.5) * 2.0);

    vec4 res = kaliset(p);
    vec2 r = res.xy;
    float d = res.w;

    vec3 color = palette(d,
        vec3(0.5, 0.5, 0.5), 
        vec3(0.5, 0.5, 0.5),
        vec3(1.0, 1.0, 1.0),
        fract(vec3(
                texture(sampler2D(spectrum, spectrum_sampler), vec2(0.7, 0)).x,
                texture(sampler2D(spectrum, spectrum_sampler), vec2(0.4, 0)).x,
                texture(sampler2D(spectrum, spectrum_sampler), vec2(0.1, 0)).x
        ))
    );

    // edge fade
    float v = (1.0 - exp((abs(uv.x) - 1.0) * 6.0)) * (1.0 - exp((abs(uv.y) - 1.0) * 6.0));
    color *= mix(0.4, 1.0, v);

    frag_color = vec4(color * color, 1);
}
