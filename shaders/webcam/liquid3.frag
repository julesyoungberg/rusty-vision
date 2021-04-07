#version 450

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 frag_color;

layout(set = 0, binding = 0) uniform GeneralUniforms {
    vec2 mouse;
    vec2 resolution;
    float time;
    int mouse_down;
};

layout(set = 1, binding = 0) uniform sampler webcam_sampler;
layout(set = 1, binding = 1) uniform utexture2D webcam;
layout(set = 1, binding = 2) uniform WebcamUniforms {
    vec2 video_size;
};

#define PI 3.14159265359

vec3 webcam_color(in vec2 coord) {
    return texture(usampler2D(webcam, webcam_sampler), coord).xyz / 255.0;
}

void main() {
    vec2 st = uv * 0.5 + 0.5;

    float t1 = time * 1.9;
    float t2 = time * 1.7;

    for (float i = 1.0; i < 3.0; i += 1.0) {
        vec2 p = st;
        p.x += 0.07 / i * sin(i * PI * st.y * 6.0 + t1 + sin(t1 * 1.11)) * cos(t1);
        p.y += 0.05 / i * cos(i * PI * st.x * 6.0 + t2 + sin(t2 * 0.77)) * cos(t2 + 1.5);
        st = p;
    }
    
    vec3 color = webcam_color(st);
    
	frag_color = vec4(color, 1.0);
}
