#version 450

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 frag_color;

layout(set = 0, binding = 0) uniform GeneralUniforms {
    vec2 mouse;
    vec2 resolution;
    float time;
};

#define PI 3.14159265359

//@import util/rand
float rand21(vec2 co);

float flower_df(in vec2 st, in float pedals) {
    float x = st.x * pedals;
    return min(fract(x), fract(1.0 - x));
}

float dots(in vec2 st, in vec2 num_dots, in float size, in float x_correction) {
    st *= num_dots;
    vec2 gv = fract(st) - 0.5;
    return smoothstep(size, size * 0.95, length(vec2(gv.x * x_correction, gv.y)));
}

void main() {
    vec2 st = uv;
    st.x *= resolution.x / resolution.y;

    float stp = 1.0 / resolution.y;

    vec3 color = vec3(0.9, 0.9, 0.75);

    // convert to polar
    st = vec2(atan(st.x, st.y) / (PI * 2.0) + 0.5, length(st));

    vec3 color1 = vec3(0.5, 0.1, 0.6);
    vec3 color2 = vec3(0.6, 0.5, 0.1);

    float shift = (sin(time * 2.5) * 0.5 + 0.5) * 0.02;

    // background dots
    vec2 sv = vec2(st.x + PI * 0.11, st.y - 0.5) * vec2(1.0, 5.0);
    vec2 dv = fract(sv) - 0.5;
    vec2 id = floor(sv);
    float dshift = max(sin(time * 4.0 + id.y * PI * 0.5) * 2.0 - 1.0, 0.0);
    float dsize = 0.3 + dshift * 0.1;
    float d = smoothstep(dsize, dsize * 0.95, length(vec2(dv.x * 30.0 * st.y, dv.y)));
    color = mix(color, vec3(0.05 + dshift * 0.5, 0, 0.6), d * (step(0.9, st.y) - step(1.5, st.y)));

    // outer flower
    vec2 c1 = vec2(st.x, st.y);
    float f1_df = flower_df(c1, 5.0);
    float radius1 = 0.6;
    float depth1 = 0.7;
    // draw outline
    float f1_outline = smoothstep(0.0, stp, f1_df * radius1 + depth1 - c1.y);
    color = mix(color, vec3(0), f1_outline);
    // draw fill
    radius1 *= 0.97 + shift;
    depth1 *= 0.97 + shift;
    float f1 = smoothstep(0.0, stp, f1_df * radius1 + depth1 - c1.y);
    color = mix(color, color1, f1);
    // dots
    float f1_dots = dots(vec2(c1.x, c1.y - 0.5), vec2(5.0, 15.0), 0.1, 10.0);
    color = mix(color, color2, f1 * f1_dots);
    // draw inner fade
    float f1_shadow = smoothstep(0.0, 0.35, f1_df * radius1 + depth1 - c1.y);
    color = mix(color, vec3(0), f1_shadow);

    // middle flower
    vec2 c2 = vec2(st.x + PI / 30.0, st.y);
    float f2_df = flower_df(c2, 5.0);
    float radius2 = 0.9;
    float depth2 = 0.55;
    // draw outline
    float f2_outline = smoothstep(0.0, stp, f2_df * radius2 + depth2 - c2.y);
    color = mix(color, vec3(0.0), f2_outline);
    // draw fill
    radius2 *= 0.97 + shift;
    depth2 *= 0.97 + shift;
    float f2 = smoothstep(0.0, stp, f2_df * radius2 + depth2 - c2.y);
    color = mix(color, color1, f2);
    // dots
    float f2_dots = dots(vec2(c2.x, c2.y - 0.5), vec2(5.0, 15.0), 0.1, 10.0);
    color = mix(color, color2, f2 * f2_dots);
    // draw inner fade
    float f2_shadow = smoothstep(0.0, 0.55, f2_df * radius2 + depth2 - c2.y);
    color = mix(color, vec3(0), f2_shadow);

    // inner flower
    float f3_df = flower_df(c1, 5.0);
    float radius3 = 0.4;
    float depth3 = 0.5;
    // draw outine
    float f3_outline = smoothstep(0.0, stp, f3_df * radius3 + depth3 - c1.y);
    color = mix(color, vec3(0.0), f3_outline);
    // draw fill
    radius3 *= 0.97 + shift;
    depth3 *= 0.97 + shift;
    float f3 = smoothstep(0.0, stp, f3_df * radius3 + depth3 - c1.y);
    color = mix(color, color1 * 1.1, f3);
    // dots
    float f3_dots = dots(vec2(c1.x, c1.y), vec2(5.0, 15.0), 0.1, 10.0);
    color = mix(color, color2, f3 * f3_dots);
    // lines
    float f3_line = smoothstep(stp * 10.0, 0.0, fract(c1.x * 5.0));
    color = mix(color, vec3(0), f3 * f3_line);
    // inner fade
    float f3_fade = smoothstep(0.8, 0.0, c1.y);
    color = mix(color, vec3(0), f3 * f3_fade);

    // inner circle
    float size = 0.2 + shift;
    float heart = smoothstep(size, size * 0.9, st.y);
    color = mix(color, color2 * 1.2, heart);
    color = mix(color, vec3(0), heart * rand21(st) * 0.6);
    
    frag_color = vec4(color, 1);
}
