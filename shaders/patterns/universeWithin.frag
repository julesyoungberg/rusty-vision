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
//@import util/rand

vec3 hsv2rgb(vec3 c);
float rand(float n);
float rand21(vec2 p);
vec2 rand2(vec2 p);

// from the Art of Code

float line_dist(vec2 p, vec2 a, vec2 b) {
    vec2 pa = p - a;
    vec2 ba = b - a;
    float t = clamp(dot(pa, ba) / dot(ba, ba), 0.0, 1.0);
    return length(pa - ba * t);
}

float line(vec2 p, vec2 a, vec2 b) {
    float d = line_dist(p, a, b);
    float m = smoothstep(0.03, 0.01, d);
    float d2 = length(a - b);
    m *= smoothstep(1.2, 0.8, d2) + smoothstep(0.05, 0.03, d2 - 0.75);
    return m;
}

vec2 get_point(vec2 id) {
    return sin(rand2(id) * time) * 0.4;
}

// draws 1 layer of the pseudo-3d effect
vec3 layer(vec2 st) {
    vec2 gv = fract(st) - 0.5;
    vec2 id = floor(st);

    vec2 points[9];
    vec2 ids[9];
    int i = 0;

    // collect neighboring points
    for (float y = -1.0; y <= 1.0; y++) {
        for (float x = -1.0; x <= 1.0; x++) {
            ids[i] = id + vec2(x, y);
            points[i++] = get_point(id + vec2(x, y)) + vec2(x, y);
        }
    }

    vec3 color = vec3(0);
    float t = time;

    // draw points and lines
    for (int j = 0; j < 9; j++) {
        color += line(gv, points[4], points[j]);

        vec2 q = (points[j] - gv) * 20.0;
        float sparkle = 1.0 / (dot(q, q) * 2.0);
        float id = rand21(ids[j]);
        vec3 c = hsv2rgb(vec3(fract(id + t * 0.1), 1, 1));
        float strength = log(texture(sampler2D(spectrum, spectrum_sampler), vec2(id, 0)).x + 1.0);
        color += sparkle * c * strength;
    }

    // draw lines that pass through the center without a point in it
    color += line(gv, points[1], points[3]);
    color += line(gv, points[1], points[5]);
    color += line(gv, points[7], points[3]);
    color += line(gv, points[7], points[5]);

    return color;
}

void main() {
    vec2 st = uv;
    st.x *= resolution.x / resolution.y;

    vec3 color = vec3(0.0);

    float gradient = st.y * 0.1;
    
    // float m = 0.0;
    float t = time * 0.1;
    vec3 base = sin(t * 5.0 * vec3(0.345, 0.456, 0.657)) * 0.4 + 0.6;

    float s = sin(t);
    float c = cos(t);
    mat2 rot = mat2(c, -s, s, c);
    st *= rot;

    for (float i = 0.0; i < 1.0; i += 0.25) {
        float z = fract(i + t);
        float size = mix(7.0, 0.5, z);
        float fade = smoothstep(0.0, 0.5, z) * smoothstep(1.0, 0.8, z);
        // m += layer(st * size + i * 20.0) * fade;
        color += layer(st * size + i * 20.0) * fade;
    }

    // color += m * base;
    // float gradient_strength = texture(sampler2D(spectrum, spectrum_sampler), vec2(0.1, 0)).x;
    // color -= gradient * base * gradient_strength;

    frag_color = vec4(color, 1);
}
