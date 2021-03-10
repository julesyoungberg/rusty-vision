#version 450

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 frag_color;

layout(set = 0, binding = 0) uniform GeneralUniforms {
    vec2 mouse;
    vec2 resolution;
    float time;
    int mouse_down;
};

#define TAU 6.28318530718

//@import util/rand

vec4 rand14(float t);
float rand(float n);

// from The Art of Code
struct ray {
    vec3 origin;
    vec3 direction;
};

ray cast_ray(vec2 uv, vec3 cam_pos, vec3 target, float zoom) {
    ray r;
    r.origin = cam_pos;

    vec3 forward = normalize(target - cam_pos);
    vec3 right = cross(vec3(0, 1, 0), forward);
    vec3 up = cross(forward, right);
    vec3 center = r.origin + forward * zoom;
    vec3 intersection = center + uv.x * right + uv.y * up;

    r.direction = normalize(intersection - cam_pos);
    return r;
}

vec3 closest_point(ray r, vec3 p) {
    return r.origin + max(0.0, dot(p - r.origin, r.direction)) * r.direction;
}

float ray_point_dist(ray r, vec3 p) {
    return length(p - closest_point(r, p));
}

float bokeh(ray r, vec3 p, float size, float blur) {
    float d = ray_point_dist(r, p);
    size *= length(p);
    float c = smoothstep(size, size * (1.0 - blur), d);
    c *= mix(0.7, 1.0, smoothstep(size * 0.8, size, d));
    return c;
}

vec3 street_lights(in ray r, float t) {
    float side = step(0.0, r.direction.x);
    r.direction.x = abs(r.direction.x);

    float mask = 0.0;
    float stp = 0.1;

    for (float i = 0.0; i < 1.0; i += stp) {
        float ti = fract(t + i + side * stp * 0.5);
        vec3 p = vec3(2.0, 2.0, 100.0 - ti * 100.0);
        mask += bokeh(r, p, 0.05, 0.1) * ti * ti;
    }

    vec3 color = vec3(1.0, 0.7, 0.3) * mask;
    return color;
}

vec3 env_lights(in ray r, float t) {
    float side = step(0.0, r.direction.x);
    r.direction.x = abs(r.direction.x);

    float stp = 0.1;
    vec3 color = vec3(0);

    for (float i = 0.0; i < 1.0; i += stp) {
        float ti = fract(t + i + side * stp * 0.5);
        
        vec4 n = rand14(i + side * 100.0);

        float occlusion = sin(ti * TAU * 10.0 * n.x) * 0.5 + 0.5;
        float x = mix(2.5, 10.0, n.x);
        float y = mix(0.1, 1.5, n.y);
        float fade = occlusion;

        vec3 p = vec3(x, y, 50.0 - ti * 50.0);
        vec3 light_color = n.wzy;
        color += bokeh(r, p, 0.05, 0.1) * fade * light_color * 0.5;
    }

    return color;
}

vec3 head_lights(in ray r, float t) {
    t *= 2.0;
    float width1 = 0.25;
    float width2 = width1 * 1.2;
    float mask = 0.0;
    float stp = 0.03;

    for (float i = 0.0; i < 1.0; i += stp) {
        float n = rand(i * 13.0);
        if (n > 0.1) {
            continue;
        }

        float ti = fract(t + i);
        float z = 100.0 - ti * 100.0;
        float fade = pow(ti, 6.0);
        float focus = smoothstep(0.9, 1.0, ti);
        float size = mix(0.05, 0.03, ti);

        mask += bokeh(r, vec3(-1.0 - width1, 0.15, z), size, 0.1) * fade;
        mask += bokeh(r, vec3(-1.0 + width1, 0.15, z), size, 0.1) * fade;

        mask += bokeh(r, vec3(-1.0 - width2, 0.15, z), size, 0.1) * fade;
        mask += bokeh(r, vec3(-1.0 + width2, 0.15, z), size, 0.1) * fade;

        float ref = 0.0;
        ref += bokeh(r, vec3(-1.0 - width2, -0.15, z), size * 3.0, 1.0) * fade;
        ref += bokeh(r, vec3(-1.0 + width2, -0.15, z), size * 3.0, 1.0) * fade;

        mask += ref * focus;
    }

    vec3 color = vec3(0.9, 0.9, 1.0) * mask;
    return color;
}

vec3 tail_lights(in ray r, float t) {
    t *= 0.25;
    float width1 = 0.25;
    float width2 = width1 * 1.2;
    float mask = 0.0;
    float stp = 0.06;

    for (float i = 0.0; i < 1.0; i += stp) {
        float n = rand(i * 17.0);
        if (n > 0.5) {
            continue;
        }

        float lane = step(0.25, n);
        float ti = fract(t + i);
        float z = 100.0 - ti * 100.0;
        float fade = pow(ti, 6.0);
        float focus = smoothstep(0.9, 1.0, ti);
        float size = mix(0.05, 0.03, ti);
        float lane_shift = smoothstep(1.0, 0.96, ti);
        float x = 1.5 - lane * lane_shift;

        mask += bokeh(r, vec3(x - width1, 0.15, z), size, 0.1) * fade;
        mask += bokeh(r, vec3(x + width1, 0.15, z), size, 0.1) * fade;

        float blink = step(0.0, sin(t * 1000.0)) * 7.0 * lane * step(0.96, ti);
        mask += bokeh(r, vec3(x - width2, 0.15, z), size, 0.1) * fade;
        mask += bokeh(r, vec3(x + width2, 0.15, z), size, 0.1) * fade * (1.0 + blink);

        float ref = 0.0;
        ref += bokeh(r, vec3(x - width2, -0.15, z), size * 3.0, 1.0) * fade;
        ref += bokeh(r, vec3(x + width2, -0.15, z), size * 3.0, 1.0) * fade * (1.0 + blink * 0.1);

        mask += ref * focus;
    }

    vec3 color = vec3(1.0, 0.1, 0.03) * mask;
    return color;
}

vec2 rain(in vec2 st, float t) {
    t *= 40.0;
    vec2 a = vec2(3.0, 1.0);
    vec2 uv = st * a;
    vec2 id = floor(uv);
    uv.y += t * 0.21;

    float n = fract(sin(id.x * 716.34) * 749.34); // random
    uv.y += n;
    st.y += n;
    vec2 gv = fract(uv) - 0.5;
    id = floor(uv);

    // main rain drop
    t += fract(sin(id.x * 716.34 + id.y * 1453.7) * 749.34) * TAU; // random
    float x_shift = fract(sin(id.x * 716.34 + id.y * 1453.7) * 749.34) * 0.4;
    gv += x_shift;
    float y = -sin(t + sin(t + sin(t) * 0.5)) * 0.43;
    vec2 p1 = vec2(0, y);
    vec2 o1 = (gv - p1) / a;
    float d = length(o1);
    float m1 = smoothstep(0.07, 0.0, d);

    // left behind trickle
    vec2 gv2 = fract(st * a.x * vec2(1.0, 2.0)) - 0.5;
    gv2.x += x_shift;
    vec2 o2 = gv2 / vec2(1.0, 2.0);
    d = length(o2);
    float m2 = smoothstep(0.2 * (0.5 - gv.y + 0.5), 0.0, d) * smoothstep(-0.1, 0.1, gv.y - p1.y);

    return vec2(m1 * o1 * 30.0 + m2 * o2 * 10.0);
}

void main() {
    vec2 st = uv;
    st.x *= resolution.x / resolution.y;

    float t = time * 0.1;
    vec2 rain_distortion = rain(st * 2.0, t) * 0.5;
    rain_distortion += rain(st * 5.0, t) * 0.5;
    st -= rain_distortion * 0.5;

    vec3 cam_pos = vec3(0.5, 0.2, 0);
    vec3 target = vec3(0.5, 0.2, 1.0);

    st.x += sin(st.y * 70.0) * 0.005;
    st.y += sin(st.x * 170.0) * 0.003;
    ray r = cast_ray(st, cam_pos, target, 2.0);

    vec3 color = street_lights(r, t);
    color += env_lights(r, t);
    color += head_lights(r, t);
    color += tail_lights(r, t);
    color += (r.direction.y + 0.25) * 0.1 * vec3(0.2, 0.1, 0.5);

    frag_color = vec4(color, 1.0);
}
