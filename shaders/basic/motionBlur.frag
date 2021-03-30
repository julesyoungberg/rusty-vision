#version 450

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 frag_color;

layout(set = 0, binding = 0) uniform GeneralUniforms {
    vec2 mouse;
    vec2 resolution;
    float time;
    int mouse_down;
};

layout(set = 1, binding = 0) uniform sampler multipass_sampler;
layout(set = 1, binding = 1) uniform texture2D pass1;
layout(set = 1, binding = 2) uniform MultipassUniforms {
    int pass_index;
};

void main() {
    vec2 st = uv;
    st.x *= resolution.x / resolution.y;
    
    vec3 color = vec3(0);

    float t = time * 2.0;

    vec2 light_pos = vec2(cos(t), sin(t)) * 0.8;

    float d = distance(st, light_pos);
    // color += 0.01 / d;

    color += smoothstep(0.1, 0.0, d);
    // color += pass_index;
    vec3 prev_color = texture(sampler2D(pass1, multipass_sampler), uv * 0.5 + 0.5).rgb;
    color = max(prev_color, color);

    frag_color = vec4(color, 1);
}
