#version 450

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 frag_color;

layout(set = 0, binding = 0) uniform GeneralUniforms {
    vec2 mouse;
    vec2 resolution;
    float time;
};

#define PI 3.14159265359

float Xor(float a, float b) {
    return a * (1.0 - b) + b * (1.0 - a);
}

void main() {
    vec2 st = uv;
    st.y *= resolution.y / resolution.x;

    float angle = PI / 4.0; // + length(st) * sin(time * 0.1);
    float c = cos(angle);
    float s = sin(angle);
    st *= mat2(c, -s, s, c);
    st *= 20.0;

    vec3 color = vec3(0);

    vec2 gv = fract(st) - 0.5; 
    vec2 id = floor(st);
    float m = 0;
    float t = time * -1.5;

    for (float y = -1.0; y <= 1.0; y++) {
        for (float x = -1.0; x <= 1.0; x++) {
            vec2 offset = vec2(x, y);
            vec2 lid = id + offset;
            vec2 lgv = gv - offset;

            angle = time;
            c = cos(angle);
            s = sin(angle);
            lgv *= mat2(c, -s, s, c);

            float center_dist = length(lid) * 0.3;
            float r = mix(0.1, 1.5, pow(sin(t + center_dist), 2.0) * 0.5 + 0.5);
            float circle_dist = length(lgv);
            m = Xor(smoothstep(r, r * 0.95, circle_dist), m);
        }
    } 

    // color += m;
    color = sin(time * 0.5 + m * vec3(0.1, PI * 1.8, PI * 4.7) + length(gv)) * 0.5 + 0.5;

    frag_color = vec4(color, 1);
}
