#version 450

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 frag_color;

layout(set = 0, binding = 0) uniform GeneralUniforms {
    vec2 mouse;
    vec2 resolution;
    float time;
    int mouse_down;
};

layout(set = 1, binding = 0) uniform sampler image_sampler;
layout(set = 1, binding = 1) uniform texture2D image1;
layout(set = 1, binding = 2) uniform texture2D image2;
layout(set = 1, binding = 2) uniform ImageUniforms {
    vec2 image1_size;
    vec2 image2_size;
};

#define PI 3.14159265359

// based on https://www.shadertoy.com/view/llGcRK
void main() {
    vec2 st = uv * resolution / resolution.y;
    // st *= cos(time * 0.5) + 1.5;

    vec3 color = vec3(0.0);

    float scale = PI / 3.0;

    for (float i = 0.0; i < 3; i += 1.0) {
        float scaleFactor = i; // + sin(time * 0.05) + 1.5;

        float angle = time * scaleFactor * 0.01;
        st *= mat2(cos(angle + PI * 0.25 * vec4(0, 6, 2, 0)));

        float theta = atan(st.x, st.y) + PI;
        theta = (floor(theta / scale) + 0.5) * scale;

        vec2 dir = vec2(sin(theta), cos(theta));
        vec2 codir = dir.yx * vec2(-1, 1);

        st = vec2(dot(dir, st), dot(codir, st));
        st.xy += vec2(sin(time * 0.5), cos(time * 0.7)) * scaleFactor * 0.035;
        st = abs(fract(st + 0.5) * 2.0 - 1.0) * 0.7;
    }

    color = texture(sampler2D(image1, image_sampler), mod(st * 0.5 - time * 0.05, 1.0)).xyz;

	frag_color = vec4(color, 1.0);
}
