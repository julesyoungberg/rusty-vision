#version 450

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 frag_color;

layout(set = 0, binding = 0) uniform GeneralUniforms {
    vec2 mouse;
    vec2 resolution;
    float time;
    int mouse_down;
};

layout(set = 1, binding = 0) uniform sampler video_sampler;
layout(set = 1, binding = 1) uniform utexture2D video;
layout(set = 1, binding = 2) uniform VideoUniforms {
    vec2 video_size;
};

void main() {
    vec2 st = uv * 0.5 + 0.5;
    
    vec3 color = texture(usampler2D(video, video_sampler), st).xyz / 255.0;
    
	frag_color = vec4(color, 1.0);
}
