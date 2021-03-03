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

// reference https://www.shadertoy.com/view/llSyDh
const vec2 s = vec2(1, 1.7320508);

mat2 r2(in float a) { float c = cos(a), s = sin(a); return mat2(c, -s, s, c); }

float hex_dist(in vec2 p) {
    p = abs(p);
    return max(dot(p, normalize(vec2(1.0, sqrt(3)))), p.x);
}

vec4 hex_coords(in vec2 st) {
    vec2 r = vec2(1, sqrt(3));
    vec2 h = r * 0.5;

    vec2 a = mod(st, r) - h;
    vec2 b = mod(st - h, r) - h;

    vec2 gv = length(a) < length(b) ? a : b;
    vec2 id = st - gv;

    return vec4(gv, id);
}

float hash21(vec2 p) {
    p = fract(p * vec2(234.34, 435.345));
    p += dot(p, p);
    return fract(p.x * p.y);
}

// Polar coordinate of the arc pixel.
float polar_coord(vec2 q, float dir) {
    // The actual animation. You perform that before polar conversion.
    q = r2(time * dir) * q;
    // Polar angle.
    float a = atan(q.y, q.x);
    // Wrapping the polar angle.
    return mod(a / PI, 2.0) - 1.0;
}

// Dot pattern.
float dots(in vec2 p) {
    return length(abs(fract(p) - 0.5));
}

void main() {
    vec2 st = uv;
    st.y *= resolution.y / resolution.x;

    vec3 color = vec3(0);

    // st += time * 0.05;
    st *= 6.0;

    vec4 coords = hex_coords(st);
    vec2 gv = coords.xy;
    vec2 id = coords.zw;
    float edge_dist = 0.5 - hex_dist(gv);

    float h = hash21(id + 0.001);
    if (h < 0.5) gv.y *= -1.0;

    const float r = 1.0;
    const float th = 0.2;
    vec2 q;

    // Arc one.
    q = gv - vec2(0, r) / s;
    vec3 da = vec3(q, length(q));
    
    // Arc two.
    q = r2(PI * 2.0 / 3.0) * gv - vec2(0, r) / s;
    vec3 db = vec3(q, length(q));

    // Arc three. 
    q = r2(PI * 4.0 / 3.0) * gv - vec2(0, r) / s;
    vec3 dc = vec3(q, length(q));
    
    // Compare distance fields, and return the vector used to produce the closest one.
    vec3 q3 = (da.z < db.z && da.z < dc.z) ? da : (db.z < dc.z) ? db : dc;

    q3.z -= 0.57735 / 2.0 + th / 2.0;
    q3.z = max(q3.z, -th - q3.z);

    float d = q3.z;
    float sd = floor(mod(d * 25.0 - time, 3));
    // color += sd / 3.0;

    vec3 color1 = vec3(0.95, 0.32, 0.06);
    vec3 color2 = vec3(0.87, 0.71, 0.28);
    vec3 color3 = vec3(0.03, 0.26, 0.34);

    // background / distance field coloring
    color = mix(mix(color1, color2, sd), color3, sd - 1.0);

    float width = 0.1;
    float mask = smoothstep(0.015, 0.0, d);

    // faux 3d edge shading
    color = mix(color, color * smoothstep(0.1, -0.1, d), mask);

    // uv coords
    float a = polar_coord(q3.xy, 1.0);
    vec2 t_uv = vec2(q3.z + 1.0, a);
    float d2 = min(dots(t_uv * 4.5), dots(t_uv * 4.5 + 0.5)) - 0.3;

    // poka dots
    color = mix(color, vec3(0), mask * (1.0 - smoothstep(0.0, 0.02, d2)));
    color = mix(color, vec3(1.0, 0.4, 0.4), mask * (1.0 - smoothstep(0.0, 0.02, d2 + 0.125)));

    frag_color = vec4(color, 1);
}
