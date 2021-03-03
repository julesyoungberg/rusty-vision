#version 450

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 frag_color;

layout(set = 0, binding = 0) uniform GeneralUniforms {
    vec2 mouse;
    vec2 resolution;
    float time;
    int mouse_down;
};

#define PI 3.14159265359

float Xor(float a, float b) {
    return a * (1.0 - b) + b * (1.0 - a);
}

void main() {
    vec2 st = uv;
    st.y *= resolution.y / resolution.x;

    float angle = PI / 4.0;
    float c = cos(angle);
    float s = sin(angle);
    st *= mat2(c, -s, s, c);
    
    vec3 color = vec3(0);
    
    for (float i = 0; i < 3; i++) {
        vec2 p = st;

        float shift = sin(i * PI + time * (i + 0.1) + length(p) * 5.0) * 0.01;
        p += shift;

        p *= 20.0;

        vec2 gv = fract(p) - 0.5; 
        vec2 id = floor(p);
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
                float r = mix(0.5 , 1.5, sin(t + center_dist) * 0.5 + 0.5);
                float circle_dist = length(lgv);
                m = Xor(smoothstep(r, r * 0.95, circle_dist), m);
            }
        } 

        color[int(i)] += m;
    }

    frag_color = vec4(color, 1);
}
