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

#define AUDIO_REACTIVE 1

//@import util/hsv2rgb
//@import util/above_line

vec3 hsv2rgb(vec3 c);

bool above_line(vec2 r, vec2 q, vec2 p);

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

    vec2 id = st - gv;

    return vec4(gv, id);
}

void main() {
    vec2 st = uv;
    st.y *= resolution.y / resolution.x;

    vec3 color = vec3(0);

    st *= 5.0;

    vec4 coords = hex_coords(st);
    vec2 gv = coords.xy;
    vec2 id = coords.zw;

    float i = dot(id, id + vec2(13, 17));

    vec2 center = vec2(0, 0);
    vec2 right = vec2(sqrt(3.0) * 0.5, 0.5);
    vec2 left = vec2(-sqrt(3.0) * 0.5, 0.5);
    vec2 bottom = vec2(0.0, -1.0);

    bool cr = above_line(center, right, gv);
    bool cl = above_line(center, left, gv);
    bool cb = above_line(center, bottom, gv);

    float hue = 0.0;
    if (cr && !cl) { // top
        hue = 0.0;
    } else if (!cr && cb) { // right 
        hue = 0.33;
    } else if (cl && !cb) { // left
        hue = 0.66;
    }

    color = hsv2rgb(vec3(
        mod(hue + time * 0.2 * (hue + 0.5) + i * 0.4, 1), 
        0.6 + sin(time * 0.13 * (hue + 0.5) + i * 0.3 + hue * 2.23) * 0.3, 
        0.7 + sin(time * 0.11 * (hue + 0.5) + i * 0.7 + hue * 3.55) * 0.2
    ));

    if (AUDIO_REACTIVE == 1) {
        float intensity = texture(sampler2D(spectrum, spectrum_sampler), vec2(mod(dot(id, id) * 0.1 + hue + time * 0.01, 1), 0)).x;
        color *= clamp(log(intensity * 2.0), 0.3, 1.1);
    }

    frag_color = vec4(color, 1);
}
