#version 450

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 frag_color;

layout(set = 0, binding = 0) uniform GeneralUniforms {
    vec2 mouse;
    vec2 resolution;
    float time;
};

#define PI 3.14159265359

float flower(in vec2 st, in float pedals, in float radius, in float depth) {
    float x = st.x * pedals;
    float m = min(fract(x), fract(1.0 - x));
    return smoothstep(0.0, 0.05, m * radius + depth - st.y);
}

float dots(in vec2 st, in float num_dots, in float y_shift) {
    st *= num_dots;
    float x = fract(st.x) - 0.5;
    return smoothstep(0.21, 0.2, length(vec2(x * 2.0, st.y - y_shift * num_dots)));
}

void main() {
    vec2 st = uv;
    st.x *= resolution.x / resolution.y;

    vec3 color = vec3(0);

    // convert to polar
    st = vec2(atan(st.x, st.y) / (PI * 2.0) + 0.5, length(st));

    vec2 c1 = vec2(st.x + time * 0.05, st.y);
    float f1 = flower(c1, 5.0, 0.5, 0.7);
    color += vec3(0.5, 0, 0.7) * f1;

    float f2 = flower(vec2(st.x - time * 0.1, st.y), 7.0, 0.2, 0.5);
    color = mix(color, vec3(0.75, 0.5, 0), f2);

    float f3 = flower(vec2(st.x + time * 0.01, st.y), 10.0, sin(time * 2.0) * 0.2 + 0.2, 0.2);
    color = mix(color, vec3(0.0, 0.0, 0.7), f3);

    color -= dots(c1, 10.0, 0.65) * vec3(1, 0, 0);
    
    frag_color = vec4(color, 1);
}
