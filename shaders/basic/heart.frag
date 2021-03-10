#version 450

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 frag_color;

layout(set = 0, binding = 0) uniform GeneralUniforms {
    vec2 mouse;
    vec2 resolution;
    float time;
    int mouse_down;
};

// from the art of code
#define HEART_COLOR vec3(1.0, 0.05, 0.05)

//@import util/smoothmax

float smoothmax(float a, float b, float k);

float heart(in vec2 st, float blur) {
    float radius = 0.25;
    blur *= radius;

    st.x *= 0.7;
    st.y -= smoothmax(sqrt(abs(st.x)) * 0.5, blur, 0.1);
    st.y += 0.1 + blur * 0.5;

    float d = length(st);
    float c = smoothstep(radius + blur, radius - blur, d);   

    return c;
}

void main() {
    vec2 st = uv;
    st.x *= resolution.x / resolution.y;

    vec3 color = vec3(0.0);

    vec2 m = mouse * 2.0 / resolution.y;
    m = m * 0.5 + 0.5;

    float blur = m.y;

    float c = heart(st, blur);

    color += c * HEART_COLOR;

    frag_color = vec4(color, 1);
}
