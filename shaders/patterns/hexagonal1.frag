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

//@import util/hsv2rgb

vec3 hsv2rgb(vec3 c);

float hex_dist(in vec2 p) {
    p = abs(p);
    return max(dot(p, normalize(vec2(1.0, sqrt(3)))), p.x);
}

vec4 hex_coords(in vec2 st) {
    vec2 r = vec2(1, sqrt(3));
    vec2 h = r * 0.5;

    vec2 a = mod(st, r) - h;
    vec2 b = mod(st - h, r) - h;

    vec2 gv = length(a) < length(b) ? a : b;

    float x = atan(gv.x, gv.y);
    float y = 0.5 - hex_dist(gv);
    vec2 id = st - gv;

    return vec4(x, y, id);
}

void main() {
    vec2 st = uv;
    st.y *= resolution.y / resolution.x;

    vec3 color = vec3(0);

    st *= 10.0;

    vec4 coords = hex_coords(st);
    vec2 gv = coords.xy;
    vec2 id = coords.zw;

    float i = dot(id, id);

    float intensity = texture(sampler2D(spectrum, spectrum_sampler), vec2(mod(i * 0.1, 1), 0)).x;

    float d = smoothstep(0.01, 0.03, gv.y * sin(i + time)); // * intensity * 0.5
    // color += c;

    color = d * hsv2rgb(vec3(sin(i + time * 0.1), 1, 1)).zxy * log(intensity * 10.0);
    // color = mix(vec3(0), color, smoothstep(0.05, 0.06, m_edge_dist));

    frag_color = vec4(color, 1);
}
