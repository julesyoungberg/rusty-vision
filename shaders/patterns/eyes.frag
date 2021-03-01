#version 450

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 frag_color;

layout(set = 0, binding = 0) uniform GeneralUniforms {
    vec2 mouse;
    vec2 resolution;
    float time;
};

#define PI 3.14159265359

//@import util/hsv2rgb
//@import util/noise
//@import util/rand

vec3 hsv2rgb(vec3 c);
float noise2(in vec2 p);
float noise3(in vec3 p);
float rand21(vec2 co);
float rand31(vec3 co);

const vec2 s = vec2(1, 1.7320508);

// shane's hexagonal tiling (https://www.shadertoy.com/view/llSyDh)
vec4 get_hex(vec2 p) {
    vec4 hC = floor(vec4(p, p - vec2(.5, 1))/s.xyxy) + .5;
    vec4 h = vec4(p - hC.xy*s, p - (hC.zw + .5)*s);
    return dot(h.xy, h.xy)<dot(h.zw, h.zw) ? vec4(h.xy, hC.xy) : vec4(h.zw, hC.zw + vec2(.5, 1));
}

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

vec3 eye_color(in vec2 st, in vec2 id, in float closed) {
    st *= 3.5;
    vec3 color = vec3(1);

    vec2 shift = vec2(noise3(vec3(time, id * 7.67)) * 0.3, noise3(vec3(time, id * 11.94)) * 0.3);
    vec2 ev = st + shift;
    float r = length(ev);
    float a = atan(ev.y, ev.x);

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

    float iris_radius = 0.9;
    if (r < iris_radius) {
        // eye color
        color = fract(vec3(0.0 + sin(id.x * 3.77), 0.3 + sin(id.y * 5.14), 0.4 + sin((id.x + id.y) * 7.35)));
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
        f = smoothstep(iris_radius, 0.5, r);
        color *= f;
        // pupil
        f = smoothstep(0.2, 0.25, r * (noise3(vec3(time * 0.5, id)) * 0.5 + 0.5));
        color *= f;
        // fake reflection / shine
        f = smoothstep(0.5, 0.0, length(st - vec2(0.24, 0.2)));
        color += f * vec3(1.0, 0.9, 0.8) * 0.9;
        // edge smoothing
        f = smoothstep(iris_radius - 0.05, iris_radius, r);
        color = mix(color, bg, f);
    }

    r = length(st);
    a = atan(st.y, st.x);

    // corners fade
    f = smoothstep(1.8, 1.0, r);
    color *= f;

    // draw eyelids
    float eyelid = pow(cos(st.x * PI * 0.5) * 0.5 + 0.5, 0.5);
    float blink = smoothstep(0.05, 0.0, abs(noise3(vec3(time * 0.5, id)) - 0.5));
    eyelid = mix(eyelid, 0.0, blink + closed);
    float top_eyelid = smoothstep(0.2, 0.0, eyelid - st.y);
    float bottom_eyelid = smoothstep(0.2, 0.0, eyelid + st.y);
    color = mix(color, vec3(0), top_eyelid + bottom_eyelid);
    return color;
}

void main() {
    vec2 st = uv;
    st.y *= resolution.y / resolution.x;

    float angle = tan(s.x / s.y);
    float co = cos(angle);
    float si = sin(angle);
    st *= mat2(co, -si, si, co);

    vec3 color = vec3(0);

    st *= 5.0;

    vec4 coords = get_hex(st);

    vec2 gv = coords.xy;
    vec2 id = coords.zw;

    angle = -tan(s.x / s.y);
    co = cos(angle);
    si = sin(angle);
    gv *= mat2(co, -si, si, co);
    float closed = smoothstep(0.5, 0.6, noise3(vec3(time * 0.1, id * 15.369)));
    color = eye_color(gv, id, closed);

    frag_color = vec4(color, 1);
}
