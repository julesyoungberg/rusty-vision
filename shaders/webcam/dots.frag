#version 450

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 frag_color;

layout(set = 0, binding = 0) uniform GeneralUniforms {
    vec2 mouse;
    vec2 resolution;
    float time;
};

layout(set = 1, binding = 0) uniform sampler webcam_sampler;
layout(set = 1, binding = 1) uniform utexture2D webcam;
layout(set = 1, binding = 2) uniform WebcamUniforms {
    vec2 video_size;
};

//@import util/get_luminance

float get_luminance(vec3 rgb);

void main() {
    vec2 st = uv * 0.5 + 0.5;
    vec3 color = vec3(0);

    // tile the space
    float scale = 150.0;
    vec2 p = st;
    p.y *= resolution.y / resolution.x;
    p *= scale;
    vec2 gv = fract(p) - 0.5;
    vec2 id = floor(p);

    // draw circle in each grid cell
    float r = 0.4;
    float d = smoothstep(r, r * 0.95, length(gv));
    
    // get corresponding pixel brightness
    vec2 coord = (id + 0.5) / scale;
    coord.y *= resolution.x / resolution.y;
    vec3 image_color = texture(usampler2D(webcam, webcam_sampler), coord).xyz / 255.0;
    float brightness = get_luminance(image_color);

    // reduce number of shades
    float n_shades = 5.0;
    float shade = floor(mod(brightness * n_shades, n_shades)) / n_shades;

    color += d * shade;
    
	frag_color = vec4(color, 1.0);
}
