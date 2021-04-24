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
layout(set = 1, binding = 1) uniform texture2D webcam;
layout(set = 1, binding = 2) uniform WebcamUniforms {
    vec2 video_size;
};

vec3 webcam_color(in vec2 coord) {
    vec2 c = vec2(coord.x, 1.0 - coord.y);
    return texture(sampler2D(webcam, webcam_sampler), fract(c)).rgb;
}

// sobel filter https://en.wikipedia.org/wiki/Sobel_operator
void main() {
    vec2 st = uv * 0.5 + 0.5;

    vec3 color = vec3(0.0);

    const float d = 0.004;
    vec3 stp = vec3(-d, 0, d);
    float c00 = length(webcam_color(st + stp.xx));
    float c01 = length(webcam_color(st + stp.xy));
    float c02 = length(webcam_color(st + stp.xz));
    float c10 = length(webcam_color(st + stp.yx));
    float c12 = length(webcam_color(st + stp.yz));
    float c20 = length(webcam_color(st + stp.zx));
    float c21 = length(webcam_color(st + stp.zy));
    float c22 = length(webcam_color(st + stp.zz));

    float gx = c00 + 2.0 * c01 + c02 - c20 - 2.0 * c21 - c22;
    float gy = c00 + 2.0 * c10 + c20 - c02 - 2.0 * c12 - c22;
    float g = sqrt(gx * gx + gy * gy);

    color = vec3(0);
    color += step(1.0, g);
    
	frag_color = vec4(color, 1.0);
}

