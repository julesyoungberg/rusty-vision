#version 450

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 frag_color;

layout(set = 0, binding = 0) uniform GeneralUniforms {
    vec2 mouse;
    vec2 resolution;
    float time;
    int mouse_down;
};

// based on Webcam 'Giant in a Lake' by BenWheatley
// https://www.shadertoy.com/view/lt3fW8
vec3 lake_reflection(in vec2 st) {
    const float mirror_pos = -0.1;
    vec2 pixel_size = vec2(1.0, 1.0) / resolution;

    if (st.y > mirror_pos) {
        return vec3(st, 0.0);
    }

    float d = mirror_pos - st.y;
    float sine = sin(log(d) * 20.0 - time * 2.0);
    float dy = 100.0 * sine;
    float dx = 0.0;
    dy *= d;

    vec2 offset = pixel_size * vec2(dx, dy);
    vec2 tex_st = st + offset;
    tex_st.y = mirror_pos - tex_st.y;

    float shine = (sine + dx * 0.05) * 0.3;
    return vec3(tex_st, shine);
}

void main() {
    vec2 st = uv;
    st.x *= resolution.x / resolution.y;

    vec3 refl = lake_reflection(st);
    st = refl.xy;

    vec3 color = vec3(1.0);

    float a = atan(st.y, st.x);
    float r = length(st);
    color -= floor(mod(a * 50.0 - 1.5, 2.0));

    float size = 0.5;
    float mask = smoothstep(size, size - 0.01, length(st));
    vec3 sun = mix(vec3(0.93, 0.36, 0.42), vec3(0.93, 0.69, 0.38), (st.y + size * 0.5)  / size);
    color = mix(color, mask * sun * sun, mask);

    color += vec3(refl.z);

    frag_color = vec4(color, 1);
}
