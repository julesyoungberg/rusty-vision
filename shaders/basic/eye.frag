#version 450

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 frag_color;

layout(set = 0, binding = 0) uniform GeneralUniforms {
    vec2 mouse;
    vec2 resolution;
    float time;
};

#define PI 3.14159265359

//@import util/noise

float noise2(in vec2 p);

mat2 m = mat2(0.8, 0.6, -0.6, 0.8);

float fbm(in vec2 p) {
    float f = 0.0;
    f += 0.500 * noise2(p);
    p *= m * 2.02;
    f += 0.250 * noise2(p);
    p *= m * 2.03;
    f += 0.125 * noise2(p);
    p *= m * 2.01;
    f += 0.0625 * noise2(p);
    p *= m * 2.04;
    f /= 0.9375;
    return f * 0.5 + 0.5;
}

void main() {
    vec2 st = uv;
    st.x *= resolution.x / resolution.y;

    vec3 color = vec3(1);

    float r = length(st);
    float a = atan(st.y, st.x);

    float ss = sin(time * 2.0) * 0.5 + 0.5;
    float anim = 1.0 + 0.1 * ss * clamp(1.0 - r, 0.0, 1.0);
    r *= anim;

    // domain distortion
    float a2 = a + fbm(st * 20.0) * 0.05;
    // blood vessels
    float concentration = smoothstep(1.0, 1.8, r) * 0.3;
    float f = smoothstep(0.6 - concentration, 1.0, fbm(vec2(r * 6.0, a2 * 50.0)));
    color = mix(color, vec3(1.0, 0.0, 0.0), f);
    vec3 bg = color;

    float iris_radius = 0.8;
    if (r < iris_radius) {
        // eye color
        color = vec3(0.0, 0.3, 0.4);
        f = fbm(st * 5.0);
        color = mix(color, vec3(0.2, 0.5, 0.4), f);
        // pupil fade
        f = smoothstep(0.5, 0.2, r);
        color = mix(color, vec3(0.9, 0.6, 0.2), f);
        // domain distortion
        a += fbm(st * 20.0) * 0.05;
        // white shards
        f = smoothstep(0.3, 1.0, fbm(vec2(r * 6.0, a * 20.0)));
        color = mix(color, vec3(1), f);
        // dark spots
        f = smoothstep(0.4, 0.9, fbm(vec2(r * 10.0, a * 15.0)));
        color *= 1.0 - f;
        // edge fading
        f = smoothstep(0.8, 0.5, r);
        color *= f;
        // pupil
        f = smoothstep(0.2, 0.25, r * (noise2(vec2(time * 0.5, 0.0)) * 0.5 + 1.0));
        color *= f;
        // fake reflection / shine
        f = smoothstep(0.5, 0.0, length(st - vec2(0.24, 0.2)));
        color += f * vec3(1.0, 0.9, 0.8) * 0.9;
        // edge smoothing
        f = smoothstep(iris_radius - 0.05, iris_radius, r);
        color = mix(color, bg, f);
    }

    // corners fade
    f = smoothstep(1.8, 1.0, r);
    color *= f;

    // draw eyelids
    float eyelid = pow(cos(st.x * PI * 0.5) * 0.5 + 0.5, 0.5);
    float blink = smoothstep(0.05, 0.0, abs(noise2(vec2(time * 0.5, 0.0)) - 0.5));
    eyelid = mix(eyelid, 0.0, blink);
    float top_eyelid = smoothstep(0.2, 0.0, eyelid - st.y);
    float bottom_eyelid = smoothstep(0.2, 0.0, eyelid + st.y);
    color = mix(color, vec3(0), top_eyelid + bottom_eyelid);

    frag_color = vec4(color, 1);
}
