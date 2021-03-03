#version 450

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 frag_color;

layout(set = 0, binding = 0) uniform GeneralUniforms {
    vec2 mouse;
    vec2 resolution;
    float time;
    int mouse_down;
};

void main() {
    vec2 st = uv * 0.5 + 0.5;
    st.y *= resolution.y / resolution.x;
    
    vec3 color = vec3(0);

    color += sin(
        st.x * 50.0 + cos(
            time + st.y * 10.0 + sin(st.x * 50.0 + time * 2.0)
        )
    ) * 2.0;

    color += cos(
        st.x * 20.0 + sin(
            time + st.y * 30.0 + cos(st.x * 60.0 + time * 3.0)
        )
    ) * 1.5;

    color += sin(
        st.x * 30.0 + cos(
            time + st.y * 16.0 + sin(st.x * 40.0 + time * 3.0)
        )
    ) * 3.0;

    color += cos(
        st.x * 10.0 + sin(
            time + st.y * 40.0 + cos(st.x * 50.0 + time * 2.0)
        )
    ) * 2.0;

    color.rg += st;

    frag_color = vec4(color, 1);
}
