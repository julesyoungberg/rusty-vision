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

layout(set = 2, binding = 0) uniform sampler spectrum_sampler;
layout(set = 2, binding = 1) uniform texture2D spectrum;

layout(set = 3, binding = 0) uniform sampler multipass_sampler;
layout(set = 3, binding = 1) uniform texture2D pass1;
layout(set = 3, binding = 2) uniform texture2D pass2;
layout(set = 3, binding = 3) uniform MultipassUniforms {
    int pass_index;
};

#define TAU 6.28318530718

vec3 webcam_color(in vec2 p) {
    return texture(usampler2D(webcam, webcam_sampler), p).xyz / 255.0;
}

// based on rainbow chrome by myro
// https://www.shadertoy.com/view/WdXczM
void rainbow(in vec2 st) {
    vec3 color = vec3(0.0);

    st *= 0.5;
    st += 0.5;

    float t = time;

    vec3 n = vec3(1.0);

    color = webcam_color(st);
    float d = length(uv) * 2.0;

    color += sin(cos(mod(color, n) * TAU + d - t) * TAU + t + 2.0 * d) * 0.5 + 0.5;
    // color /= 2.0;

    color *= vec3(
        texture(sampler2D(spectrum, spectrum_sampler), vec2(0.4, 0)).x,
        texture(sampler2D(spectrum, spectrum_sampler), vec2(0.2, 0)).x,
        texture(sampler2D(spectrum, spectrum_sampler), vec2(0.6, 0)).x
    ) * 3.0;
    color *= 1.0 + texture(sampler2D(spectrum, spectrum_sampler), vec2(0.8, 0)).x;
        
	frag_color = vec4(color, 1.0);
}

vec3 pass1_color(in vec2 p) {
    vec2 coord = vec2(p.x, 1.0 - p.y);
    return texture(sampler2D(pass1, multipass_sampler), coord).rgb;
}

// sobel filter https://en.wikipedia.org/wiki/Sobel_operator
void sobel(in vec2 st) {
    const float d = 0.004;
    vec3 stp = vec3(-d, 0, d);
    float c00 = length(pass1_color(st + stp.xx));
    float c01 = length(pass1_color(st + stp.xy));
    float c02 = length(pass1_color(st + stp.xz));
    float c10 = length(pass1_color(st + stp.yx));
    float c12 = length(pass1_color(st + stp.yz));
    float c20 = length(pass1_color(st + stp.zx));
    float c21 = length(pass1_color(st + stp.zy));
    float c22 = length(pass1_color(st + stp.zz));

    float gx = c00 + 2.0 * c01 + c02 - c20 - 2.0 * c21 - c22;
    float gy = c00 + 2.0 * c10 + c20 - c02 - 2.0 * c12 - c22;
    float g = sqrt(gx * gx + gy * gy);

    frag_color = vec4(0.0, 0.0, 0.0, 1.0);
    frag_color.rgb += pass1_color(st);
    frag_color.rgb += step(1.0, g);
}

void main() {
    if (pass_index == 0) {
        rainbow(uv);
    } else {
        sobel(uv * 0.5 + 0.5);
    }
}
