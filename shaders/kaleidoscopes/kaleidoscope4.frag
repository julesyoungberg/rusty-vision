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

// based on p6mm kaleidoscope by truename
// https://www.shadertoy.com/view/XdVcRW
#define SQ3 1.7320508076

mat2 rot2d(float a) { return mat2(cos(a),-sin(a),sin(a),cos(a)); }

vec2 p6mm(in vec2 uv, float repeats) {
    uv.x /= SQ3;
    uv = fract(uv * repeats - 0.5) - 0.5;
    uv.x *= SQ3;

    uv = abs(uv);
    
    vec2 st = uv;

    vec2 uv330 = rot2d(radians(330.0)) * uv;
    if (uv330.x < 0.0){
        st.y = (st.y - 0.5) * -1.0;
        st.x *= SQ3;
        return st * 2.0;
    }

    return uv;
}

void main() {
    vec2 st = uv;
    st.x *= resolution.x / resolution.y;

    vec3 color = vec3(0.0);

    st = p6mm(uv, 2.0);
    color.rg = st;

    frag_color = vec4(color, 1);
}
