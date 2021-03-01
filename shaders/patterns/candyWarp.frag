#version 450

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 frag_color;

layout(set = 0, binding = 0) uniform GeneralUniforms {
    vec2 mouse;
    vec2 resolution;
    float time;
};

// based on: http://glslsandbox.com/e#38710.0 & https://editor.isf.video/shaders/5e7a802d7c113618206dec38
void main() {
    vec2 st = uv;
    st *= resolution * 0.5;

    vec3 color = vec3(0.0);

    float scale = resolution.y / 50.0;
    float radius = resolution.x * 2.5;
    float gap = scale * 0.9;

    float d = length(st);
    float t = time * 1.3;

    float loops = 50.0;
    float mouse_x = 2.0 * mouse.x / resolution.x * 0.5 + 0.5;
    // modulate the distance
    d += 2.0 * mouse_x * (
        sin(st.y * 0.3 / scale + t) * sin(st.x * 0.2 / scale + t * 0.5)
    ) * scale * 5.0;
    float v = mod(d + radius / (loops * 2.0), radius / loops);
    v = abs(v - radius / (loops * 2.0));
    v = clamp(v - gap, 0.0, 1.0);
    d /= radius;

    vec3 m = fract((d - 1.0) * vec3(
        loops * sin(time * 0.01),
        loops * sin(time * 0.011 + 1.5),
        loops * sin(time * 0.009 + 3.14)
    ) * 0.5);
    color = m * v;

    frag_color = vec4(color, 1);
}
