#version 450

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 frag_color;

layout(set = 0, binding = 0) uniform GeneralUniforms {
    vec2 mouse;
    vec2 resolution;
    float time;
    int mouse_down;
};

#define PI 3.14159265359

//@import util/above_line
//@import util/line_dist

bool above_line(vec2 r, vec2 q, vec2 p);
float line_dist(vec2 p, vec2 a, vec2 b);

void main() {
    vec2 st = uv;
    st.x *= resolution.x / resolution.y;
    st *= 1.5;

    const vec2 center = vec2(0.0, 0.375);
    const float spacing = 0.31;
    const vec2 lines[3][2] = {
        { vec2(-0.5, 0.0), vec2(0.0, 0.75) }, 
        { vec2(0.0, 0.75), vec2(0.5, 0.0) },
        { vec2(0.5, 0.0), vec2(-0.5, 0.0) }
    };

    st.y += center.y;
    
    vec3 color = vec3(0.0);

    float t = time * 0.5;

    for (int i = 0; i < 3; i++) {
        // get line data
        vec2 line_start = lines[i][0];
        vec2 line_end = lines[i][1];
        vec2 line_center = (line_start + line_end) / 2.0;
        vec2 from_cent = normalize(line_center - center);

        // rotate around center
        float angle = (sin(t) * 0.5 + 0.5) * PI;
        float c = cos(angle);
        float s = sin(angle);
        mat2 rot = mat2(c, -s, s, c);
        line_start -= line_center;
        line_end -= line_center;
        line_start *= rot;
        line_end *= rot;
        line_start += line_center;
        line_end += line_center;

        // move away from center
        line_start += from_cent * spacing;
        line_end += from_cent * spacing;

        // draw line as a light bar
        float d = line_dist(st, line_start, line_end);
        float m = 0.005 / d;
        color += m * float(above_line(st, line_start, line_end));
    }

    frag_color = vec4(color, 1);
}
