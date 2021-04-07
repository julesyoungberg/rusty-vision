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

vec3 webcam_color(in vec2 coord) {
    return texture(usampler2D(webcam, webcam_sampler), coord).xyz / 255.0;
}

void main() {
    vec2 st = uv * 0.5 + 0.5;
    
    vec3 color = webcam_color(st);
    
	frag_color = vec4(color, 1.0);
}
