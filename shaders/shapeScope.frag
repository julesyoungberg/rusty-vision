#version 450

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 frag_color;

layout(set = 0, binding = 0) uniform GeneralUniforms {
    vec2 mouse;
    vec2 resolution;
    float time;
};

#define PI 3.14159265359

// https://www.shadertoy.com/view/Xl2yDW
float sdBox(in vec2 p, in vec2 b) {
    vec2 d = abs(p) - b;
    return length(max(d, 0.0)) + min(max(d.x, d.y), 0.0);
}

float square(in vec2 st, in float size) {
    float a = time + length(uv) * sin(time * 0.2) * PI * 0.5;
    float c = cos(a);
    float s = sin(a);
    mat2 rot = mat2(c, -s, s, c);
    st *= rot;
	return sdBox(st, vec2(size));
}

void main() {
    vec2 st = uv * resolution / resolution.y;
    st *= 10.0;

    float a = time + length(st) * sin(time * 0.5) * 0.2;
    float c = cos(a);
    float s = sin(a);
    mat2 rot = mat2(c, -s, s, c);
    st *= rot;

    vec2 f_st = fract(st);
    f_st -= 0.5;

    float size = 0.3 + sin(time * 0.5) * 0.1 * length(st);

    float dist = square(f_st, size);
    
    // basic shape
    vec3 color = sin(mix(vec3(1.0, 1.0, 0), vec3(1.0, 0.0, 1.0), sign(dist)) + time + length(f_st) * 0.5);
    
	frag_color = vec4(color, 1.0);
}
