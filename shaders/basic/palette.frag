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

vec3 color_round(vec3 color, float value) {
    vec3 c = color;
    c *= value;
    c = vec3(ceil(c.r), ceil(c.g), ceil(c.b));
    c /= value; // Divide it by that value
    return c;
}

void main() {
    vec2 st = uv * 0.5 + 0.5;
    
    vec3 color = vec3(0);

    float blur = 7.0;

    for (float y = -blur; y <= blur; y++) {
        for (float x = -blur; x <= blur; x++) {
            vec2 coord = st + vec2(x, y) / resolution;
            color += texture(sampler2D(image1, image_sampler), vec2(coord.x, 1.0 - coord.y)).xyz;
        }
    }

    color /= pow(blur * 2.0 + 1.0, 2.0);

    color = color_round(color, 3.0);

    vec2 m = 2.0 * mouse / resolution;

    color = fract(color + vec3(m.x, m.y, 5.49));

    frag_color = vec4(color, 1);
}
