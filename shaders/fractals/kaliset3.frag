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
//@import util/rand

vec3 palette(in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d);
vec2 rand2(vec2 p);

vec2 complex_inv(in vec2 z) {
    vec2 conjugate = vec2(z.x, -z.y);
    float denominator = dot(conjugate, conjugate);
    return conjugate / denominator;
}

vec2 complex_mult(in vec2 a, in vec2 b) {
    return vec2(a.x * b.x - a.y * b.y, a.x * b.y + a.y * b.x);
}

vec3 formula(in vec2 st, in vec2 c) {
    vec2 z = st;
    float last_stable = 0.0;
    float expsmo = 0.0;
    float l = 0.0;

    // orbit traps
    float min_comp = 1000.0;
    float min_mag = min_comp;

    float angle = time * 0.05;

    const float iterations = 40;
    for (float i = 0.0; i < iterations; i++) {
        // rotation
        //z *= mat2(cos(angle), -sin(angle), sin(angle), cos(angle));

        // original kali equation
        // z = abs(z) / dot(z, z) + c;

        // kali variations
        // z = abs(z) / (z.x * z.y) + c;
        // z = abs(complex_inv(z)) + c;
        // z = complex_mult(abs(z), complex_inv(abs(c))) + c;
        // z = abs(complex_mult(z, complex_inv(c))) + c;
        // z = abs(complex_mult(z, z)) + c;
        // z = complex_inv(complex_mult(complex_mult(z, z), z)) + c;

        // softology variations
        z.x = -abs(z.x);
        // z = complex_mult(z, c) + 1.0 + complex_inv(complex_mult(z, c) + 1.0);
        // z = abs(complex_mult(z, c) + 1.0) + complex_inv(abs(complex_mult(z, c) + 1.0));
        vec2 cone = vec2(1.0, 0.0);
        vec2 temp = abs(complex_mult(z, c) + cone);
        z = temp + complex_mult(cone, complex_inv(temp));

        float mag = length(z);

        // exponential smoothing
        if (mod(i, 2.0) < 1.0) {
            float pl = l;
            l = mag;
            expsmo += exp(-1.0 / abs(l - pl));
        }

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

    float k = clamp(expsmo * 0.06, 0.8, 1.4);

    // return vec3(expsmo * 0.1);
    last_stable += 1.0;

    float intensity = 0.01;
    float width = intensity * last_stable * 2.0;

    float circ = pow(max(0.0, width - min_mag * 0.1) / width, 6.0);
    float shape = max(pow(max(0.0, width - min_comp) / width, 0.25), circ);

    float t = time * 0.1;
    // vec3 color = vec3(rand2(z), c);
    vec3 color = palette(
        fract(expsmo * 0.05), // last_stable / iterations,
        vec3(0.5, 0.5, 0.5), 
        vec3(0.5, 0.5, 0.5),
        vec3(1.0, 1.0, 1.0),
        vec3(0.0, 0.1, 0.2)
    );
    return color;

    // carve out the pattern
    color *= 0.4 + mod(last_stable / iterations + min_mag * 0.2 - t, 1.0) * 1.6;

    // add some flare
    // circ filters out most of this addition but adds some nice highlights
    // float unstable_iterations = iterations - last_stable;
    // color += vec3(1.0, 0.7, 0.3) * circ * unstable_iterations * 3.0;

    return color * shape;
}

void main() {
    vec2 st = uv;
    st.x *= resolution.x / resolution.y;
    st *= 1.0;
    
    vec3 color = vec3(0);

    float t = time;
    // float scale = 1.0 + 0.5 * sin(t / 17.0);
    // st *= scale;

    vec2 c = vec2(-0.6);
    c = vec2(-0.355 + sin(time / 11.0) * 0.05, 0.87);
    // c += vec2(sin(t / 11.0), sin(t / 13.0)) * 0.5;

    float a = abs(atan(st.x, st.y));
    float r = length(st);

    color = formula(st, c);

    frag_color = vec4(color, 1);
}
