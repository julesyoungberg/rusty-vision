#version 450

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 frag_color;

layout(set = 0, binding = 0) uniform GeneralUniforms {
    vec2 mouse;
    vec2 resolution;
    float time;
    int mouse_down;
};

#define ITERATIONS 100.0

// from the Art of Code
void main() {
    vec2 st = uv;
    st.x *= resolution.x / resolution.y;
    
    vec3 color = vec3(0);

    vec2 m = 2.0 * mouse / resolution.y * 0.5 + 0.5;
    float zoom = pow(10.0, -m.x * 3.0);

    vec2 c = st * zoom * 3.0;
    c += vec2(-0.69955, 0.37999);

    vec2 z = vec2(0.0);
    float i = 0.0;

    float factor = 2.0 + sin(time * 0.5) * 0.5;
    float size = 2.0;
    float ma = 100.0;
    for (i = 0.0; i < ITERATIONS; i++) {
        z = vec2(pow(z.x, 2.0) - pow(z.y, 2.0), z.x * z.y * factor) + c;

        ma = min(ma, abs(z.x));

        if (length(z) > size) {
            break;
        }
    }

    float f = i / ITERATIONS;
    
    color += sqrt(f);
    color.b += ma * f;
    color.rg += z * f;

    frag_color = vec4(color, 1);
}
