#version 450

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 frag_color;

layout(set = 0, binding = 0) uniform GeneralUniforms {
    vec2 resolution;
    float time;
};

layout(set = 1, binding = 0) uniform sampler image_sampler;
layout(set = 1, binding = 1) uniform texture2D image1;
layout(set = 1, binding = 2) uniform texture2D image2;
layout(set = 1, binding = 2) uniform ImageUniforms {
    vec2 image1_size;
    vec2 image2_size;
};

void main() {
    vec2 st = uv * 0.5 + 0.5;
    
    vec3 color1 = texture(sampler2D(image1, image_sampler), st).xyz;
    vec3 color2 = texture(sampler2D(image2, image_sampler), st).xyz;

    vec3 color = mix(color1, color2, sin(time) * 0.5 + 0.5);
    
	frag_color = vec4(color, 1.0);
}
