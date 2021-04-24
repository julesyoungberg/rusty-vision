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

layout(set = 2, binding = 0) uniform sampler spectrum_sampler;
layout(set = 2, binding = 1) uniform texture2D spectrum;

//@import util/palette

vec3 palette(in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d) ;

vec3 webcam_color(in vec2 coord) {
    vec2 c = vec2(coord.x, 1.0 - coord.y);
    return texture(sampler2D(webcam, webcam_sampler), fract(c)).rgb;
}

// based on ngMir7 by netgrind
// https://www.shadertoy.com/view/MtXXRf
void main() {
    vec2 st = uv * 0.5 + 0.5;
    st += sin(time * vec2(1.0, 1.7)) * 0.01;
    
    vec3 color = webcam_color(st);

    const float steps = 4.0;

    // take max component and scale it to the step number
    float g = max(color.r, max(color.r, color.b)) * steps;

    // pattern
    float lines = 50.0;
    float f = mod((st.x + st.y + time * 0.2) * lines, 1.0);

    if (mod(g, 1.0) > f) {
        color.r = ceil(g);
    } else {
        color.r = floor(g);
    }

    color.r /= steps;

    color = palette(
        color.r,
        vec3(0.5, 0.5, 0.5), 
        vec3(0.5, 0.5, 0.5),
        vec3(0.6, 0.8, 1.5),
        fract(vec3(
                texture(sampler2D(spectrum, spectrum_sampler), vec2(0.7, 0)).x + 0.8,
                texture(sampler2D(spectrum, spectrum_sampler), vec2(0.4, 0)).x + 0.9,
                texture(sampler2D(spectrum, spectrum_sampler), vec2(0.1, 0)).x + 0.7
        ))
    );
    
	frag_color = vec4(color, 1.0);
}

