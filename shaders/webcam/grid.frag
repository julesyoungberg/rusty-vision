#version 450

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 frag_color;

layout(set = 0, binding = 0) uniform GeneralUniforms {
    vec2 mouse;
    vec2 resolution;
    float time;
    int mouse_down;
};

layout(set = 1, binding = 0) uniform sampler spectrum_sampler;
layout(set = 1, binding = 1) uniform texture2D spectrum;

layout(set = 2, binding = 0) uniform sampler webcam_sampler;
layout(set = 2, binding = 1) uniform texture2D webcam;
layout(set = 2, binding = 2) uniform WebcamUniforms {
    vec2 video_size;
};

//@import util/hsv2rgb
//@import util/rand
//@import util/rgb2hsv

vec3 hsv2rgb(vec3 c);
vec2 rand2(vec2 p);
float rand21(vec2 p);
vec3 rgb2hsv(in vec3 c);

vec3 webcam_color(in vec2 coord) {
    vec2 c = vec2(coord.x, 1.0 - coord.y);
    return texture(sampler2D(webcam, webcam_sampler), fract(c)).rgb;
}

float get_spectrum(float i) {
    return texture(sampler2D(spectrum, spectrum_sampler), vec2(fract(i), 0)).x;
}

void main() {
    vec2 st = uv;
    st.x *= resolution.x / resolution.y;

    const float scale = 4.0;

    vec3 color = vec3(0.0);

    st *= scale;
    st -= 0.5;
    vec2 gv = fract(st) - 0.5;
    vec2 id = floor(st);
    
    vec2 coord = uv * 0.5 + 0.5;
    coord += gv * 0.1 * (sin(length(id) * 0.8 - time) * 0.5 + 0.5);
    color = webcam_color(coord);

    vec3 hsv = rgb2hsv(color);
    float i = rand21(id) * 7693.78;
    color = mix(color, hsv2rgb(vec3(fract(i + time * 0.1 * fract(i)), 1.0, 1.0)), 0.3);
    color *= color;
    color *= get_spectrum(i) * 3.0 + 0.5;

	frag_color = vec4(color, 1.0);
}
