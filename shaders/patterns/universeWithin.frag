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

//@import util/hsv2rgb
//@import util/line_dist
//@import util/rand

vec3 hsv2rgb(vec3 c);
float line_dist(vec2 p, vec2 a, vec2 b);
float rand(float n);
float rand21(vec2 p);
vec2 rand2(vec2 p);

// based on The Universe Within by BigWings
// https://www.shadertoy.com/view/lscczl
// from the Art of Code

float line(vec2 p, vec2 a, vec2 b, float strength) {
    float d = line_dist(p, a, b);
    float s = mix(0.0, 0.02, strength);
    float m = smoothstep(s, 0.0, d);
    float d2 = length(a - b);
    m *= smoothstep(1.2, 0.8, d2) + smoothstep(0.05, 0.03, d2 - 0.75);
    return m;
}

vec2 get_point(vec2 id) {
    return sin(rand2(id) * time) * 0.4;
}

float get_strength(float i) {
    return log(texture(sampler2D(spectrum, spectrum_sampler), vec2(i, 0)).x + 1.0);
}

// draws 1 layer of the pseudo-3d effect
vec3 layer(vec2 st, float n) {
    vec2 gv = fract(st) - 0.5;
    vec2 id = floor(st) + n;

    vec2 points[9];
    float ids[9];
    float strengths[9];
    vec3 colors[9];
    float t = time;
    int i = 0;

    vec3 color = vec3(0);

    // collect neighboring points
    for (float y = -1.0; y <= 1.0; y++) {
        for (float x = -1.0; x <= 1.0; x++) {
            vec2 coord = id + vec2(x, y);
            points[i] = get_point(coord) + vec2(x, y);
            ids[i] = rand21(coord);
            strengths[i] = get_strength(ids[i]);
            colors[i] = hsv2rgb(vec3(fract(ids[i] + t * 0.1), 1, 1)) + 0.01;
            i++;
        }
    }

    float line_strength;
    vec3 line_color;
    float line_brightness = 0.5;

    // draw points and lines
    for (int j = 0; j < 9; j++) {
        line_strength = (strengths[4] + strengths[j]) / 2.0;
        line_color = (colors[4] + colors[j]) / 2.0 * line_brightness;
        color += line(gv, points[4], points[j], line_strength) * line_color;

        float d = length(gv - points[j]);
        float sparkle = 0.003 / (d * d);
        sparkle *= smoothstep(1.0, 0.7, d);
        color += sparkle * colors[j] * strengths[j];
    }

    // draw lines that pass through the center without a point in it
    line_strength = (strengths[1] + strengths[3]) / 2.0;
    line_color = (colors[1] + colors[3]) / 2.0 * line_brightness;
    color += line(gv, points[1], points[3], line_strength) * line_color;

    line_strength = (strengths[1] + strengths[5]) / 2.0;
    line_color = (colors[1] + colors[5]) / 2.0 * line_brightness;
    color += line(gv, points[1], points[5], line_strength) * line_color;

    line_strength = (strengths[7] + strengths[3]) / 2.0;
    line_color = (colors[7] + colors[3]) / 2.0 * line_brightness;
    color += line(gv, points[7], points[3], line_strength) * line_color;

    line_strength = (strengths[7] + strengths[3]) / 2.0;
    line_color = (colors[7] + colors[7]) / 2.0 * line_brightness;
    color += line(gv, points[7], points[5], line_strength) * line_color;

    return color;
}

void main() {
    vec2 st = uv;
    st.x *= resolution.x / resolution.y;

    vec3 color = vec3(0.0);

    float gradient = st.y * 0.1;
    
    float t = time * 0.1;
    vec3 base = sin(t * 5.0 * vec3(0.345, 0.456, 0.657)) * 0.4 + 0.6;

    float s = sin(t);
    float c = cos(t);
    mat2 rot = mat2(c, -s, s, c);
    st *= rot;

    float rot_shift = 0.5;
    s = sin(rot_shift);
    c = cos(rot_shift);
    rot = mat2(c, -s, s, c);

    for (float i = 0.0; i < 1.0; i += 0.25) {
        float z = fract(i + t);
        float size = mix(7.0, 0.5, z);
        float fade = smoothstep(0.0, 0.5, z) * smoothstep(1.0, 0.8, z);
        st *= rot;
        color += layer(st * size + i * vec2(20.0, 27.0), i) * fade;
    }

    // float gradient_strength = texture(sampler2D(spectrum, spectrum_sampler), vec2(0.1, 0)).x;
    // color -= gradient * base * gradient_strength;

    frag_color = vec4(color, 1);
}
