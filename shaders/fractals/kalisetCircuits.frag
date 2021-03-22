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

//@import util/rand

vec2 rand2(vec2 p);

// A disection of Circuits by Kali
// https://www.shadertoy.com/view/XlX3Rj
// and UltimateKaliCircuits by mojovideotech
// https://editor.isf.video/shaders/5e7a80297c113618206debea
vec4 kaliset(in vec2 st, float c) {
    vec2 z = st;
    float last_stable = 0.0;

    // orbit traps
    float min_comp = 1000.0;
    float min_mag = min_comp;

    const float iterations = 9.0;

    for (float i = 0; i < iterations; i++) {
        // kaliset formula
        z = abs(z) / clamp(dot(z, z), 0.1, 0.5) - c;

        float mag = length(z);
        float w = 0.1; // (sin(time) * 0.5 + 0.5) * 3.0;

        // get minimum component
        float m_comp = clamp(abs(min(z.x, z.y)), w - mag, abs(mag - w));
        // update overall minimum component
        min_comp = min(m_comp, min_comp);
        // update minimum magnitude
        min_mag = min(mag * 0.1, min_mag);
        // m is 0 unless minimum == min_comp
        // catches the lasst i where z is stable
        last_stable = max(last_stable, i * (1.0 - abs(sign(min_comp - m_comp))));
    }

    last_stable += 1.0;

    float intensity = 0.01;
    float width = intensity * last_stable * 2.0;

    // circ is maximal when min_mag is minimal
    // circ represents a bunch of circles at bright points of the fractal
    float circ = pow(max(0.0, width - min_mag) / width, 6.0);

    // shape is maximal when min_comp is minimal
    // shape represents the pattern of the fractal
    // circ is used here to birghten it up, as the minimum.
    float shape = max(pow(max(0.0, width - min_comp) / width, 0.25), circ);

    float t = time * 0.1;
    vec3 color = vec3(rand2(z), c);

    // carve out the pattern
    color *= 0.4 + mod(last_stable / iterations + min_mag * 2.0 - t, 1.0) * 1.6;

    // add some flare
    // circ filters out most of this addition but adds some nice highlights
    float unstable_iterations = iterations - last_stable;
    color += vec3(1.0, 0.7, 0.3) * circ * unstable_iterations * 3.0 * smoothstep(0.0, 0.5, vec3(c, st));

    return vec4(color, shape);
}

void main() {
    vec2 st = uv;
    st.x *= resolution.x / resolution.y;
    st *= 0.5;

    vec3 color = vec3(0);
    float shape = 0.0;

    float zoom = 0.2;
    st *= zoom;
    
    float t = time * 0.01;

    // move 'camera'
    float a = t;
    float b = a * 4.23;
    st *= mat2(cos(b), sin(b), -sin(b), cos(b));
    st += vec2(sin(a), cos(a * 0.5)) * 0.5;

    // what to add each iteration of the fractal equation    
	float c = 1.5 + mod(floor(t), 16.0) * 0.125;

    // anti aliasing setup
    const float samples = 16.0;
    float pix = 0.5 / resolution.x * zoom;

    // collect samples for anti aliasing
    for (float aa = 0; aa < samples; aa++) {
        vec2 aauv = floor(vec2(aa / 6.0, mod(aa, 6.0)));
        vec4 result = kaliset(st + aauv * pix, c);
        color += result.rgb;
        shape += result.a;
    }

    // normalize results
    shape /= samples;
    color /= samples;

    // carve out circuit pattern
    color = mix(vec3(0.15), color, shape);
    
    // subtle edge fade
    color *= 1.0 - length(st);

    // weight channels
    color *= vec3(1.2, 1.1, 1.0);

    frag_color = vec4(color, 1);
}
