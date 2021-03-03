#version 450

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 frag_color;

layout(set = 0, binding = 0) uniform GeneralUniforms {
    vec2 mouse;
    vec2 resolution;
    float time;
    int mouse_down;
};

float hash21(vec2 p) {
    p = fract(p * vec2(234.34, 435.345));
    p += dot(p, p);
    return fract(p.x * p.y);
}

vec3 truchet_pattern(vec2 p, float width) {
    vec2 gv = fract(p) - 0.5;
    vec2 id = floor(p);

    float n = hash21(id);
    if (n < 0.5) gv.x *= -1.0;
    
    vec2 c_uv = gv - 0.5 * sign(gv.x + gv.y + 0.001);
    float d = length(c_uv);

    float mask = smoothstep(0.01, -0.01, abs(d - 0.5) - width);

    float angle = atan(c_uv.x, c_uv.y);
    float checker = mod(id.x + id.y, 2.0) * 2.0 - 1.0;

    // float flow = sin(angle * checker * 10.0 + 2.0 * time);
    float x = fract(checker * angle / 1.57 + time);
    float y = (d - (0.5 - width)) / (width * 2.0);
    y = abs(y - 0.5) * 2.0; // mirror
    // if (n < 0.5 ^^ checker > 0.0) y = 1.0 - y; // continuous
    return vec3(x, y, mask);
}

void main() {
    vec2 st = uv;
    st.y *= resolution.y / resolution.x;

    vec3 color = vec3(0.0);

    vec2 og = st;
    
    st += time * 0.1;
    st *= 10.0;

    vec3 truchet = truchet_pattern(st, 0.2 * (1.0 - length(og)));
    vec2 t_uv = truchet.xy;
    float mask = truchet.z;
    float y = t_uv.y;
    t_uv.y *= 0.2;
    t_uv.x -= 0.5;
    t_uv *= 2.0;
    t_uv.x = fract(t_uv.x) - 0.5;

    color += mask * smoothstep(0.2, 0.21, abs(length(t_uv))) * (1.0 - y);

    // if (gv.x > 0.48 || gv.y > 0.48) color = vec3(1, 0, 0);
    
    frag_color = vec4(color, 1);
}
