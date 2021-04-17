#version 450

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 frag_color;

layout(set = 0, binding = 0) uniform GeneralUniforms {
    vec2 mouse;
    vec2 resolution;
    float time;
    int mouse_down;
};

// Image by Julie Erhart
layout(set = 1, binding = 0) uniform sampler image_sampler;
layout(set = 1, binding = 1) uniform texture2D image;
layout(set = 1, binding = 2) uniform texture2D _image2;
layout(set = 1, binding = 3) uniform ImageUniforms {
    vec2 image_size;
};

layout(set = 2, binding = 0) uniform sampler spectrum_sampler;
layout(set = 2, binding = 1) uniform texture2D spectrum;

#define PI 3.14159265359
#define AUDIO_REACTIVE 1

//@import util/complex_inv
//@import util/complex_mult
//@import util/noise
//@import util/palette
//@import util/rand

vec2 complex_inv(in vec2 z);
vec2 complex_mult(in vec2 a, in vec2 b);
float noise2(in vec2 p);
float noise3(in vec3 p);
vec3 palette(in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d);
vec2 rand2(vec2 p);

const float ball_sensitivity = 0.1;
const float bg_sensitivity = 0.05;
const float scale_strength = 0.1;
const float shake_strength = 0.05;

mat2 r2(in float a) { float c = cos(a), s = sin(a); return mat2(c, -s, s, c); }

vec4 image_color(in vec2 p, in vec2 ar) {
    vec2 image_ar = vec2(
        min(1.0, image_size.x / image_size.y),
        min(1.0, image_size.y / image_size.x)
    );
    p /= image_ar;
    p += 0.5;
    p.y = 1.0 - p.y;
    vec4 color = texture(sampler2D(image, image_sampler), p);
    return color;
}

float get_spectrum(float i) {
    return clamp(texture(sampler2D(spectrum, spectrum_sampler), vec2(fract(i), 0)).x, 0.0, 1.0);
}

// Eyes
// -------
// based on Beutypi by iq
// https://www.shadertoy.com/view/lsfGRr
mat2 m = mat2(0.8, 0.6, -0.6, 0.8);
float fbm(in vec2 p) {
    float f = 0.0;
    f += 0.500 * noise2(p);
    p *= m * 2.02;
    f += 0.250 * noise2(p);
    p *= m * 2.03;
    f += 0.125 * noise2(p);
    p *= m * 2.01;
    f += 0.0625 * noise2(p);
    p *= m * 2.04;
    f /= 0.9375;
    return f * 0.5 + 0.5;
}

vec3 eye_color(in vec2 st, in vec2 id, in vec2 shine_cent) {
    // st *= 3.5;
    vec3 color = vec3(1);
    vec2 shift = vec2(0);

    float strength = 0.0;
    if (AUDIO_REACTIVE == 1) {
        float i = fract(dot(id, id) * 0.008);
        strength = texture(sampler2D(spectrum, spectrum_sampler), vec2(i, 0)).x;
        // strength *= (i * 8.0 + 1.0);
    }

    // get a random shift
    shift = vec2(
        noise3(vec3(time * 2.0, id.x * 17.67, id.x * 230.42)),
        noise3(vec3(time * 2.0, id.x * 409.94, id.x * 97.65))
    ) * 2.0 - 1.0;
    shift = pow(shift, vec2(3.0));
    shift *= 0.8;

    // get polar coords
    vec2 ev = st + shift;
    float r = length(ev);
    float a = atan(ev.y, ev.x);

    // animate the radius of the eye
    float ss = sin(time * 2.0 + id.x * 3.77 + id.y * 5.33) * 0.1;
    float anim = 1.0 + 0.5 * ss;
    r *= anim;
    if (AUDIO_REACTIVE == 1) {
        r *= smoothstep(0.5, 0.0, strength) * 0.4 + 0.75;
    }

    // domain distortion
    float a2 = a + fbm(st * 30.0) * 0.05;
    // blood vessels
    float concentration = smoothstep(1.0, 1.8, r) * 0.3;
    float f = smoothstep(0.6 - concentration, 1.0, fbm(vec2(r * 6.0, a2 * 50.0)));
    color = mix(color, vec3(1.0, 0.0, 0.0), f);
    vec3 bg = color;

    float iris_radius = 0.9;
    if (r < iris_radius) {
        float pupil_scale = (noise3(vec3(time * 0.5, id.x * 11.11, id.y * 13.13)) * 0.65 + 0.4);
        if (AUDIO_REACTIVE == 1) {
            pupil_scale = smoothstep(0.5, 0.05, strength) * 0.5 + 0.5;
        }
        // eye color
        color = vec3(0.1, 0.6, 0.5);
        f = fbm(st * 5.0);
        color = mix(color, color + vec3(0.0, 0.2, 0.2), f);
        // pupil fade
        f = smoothstep(0.6, 0.2, r);
        color = mix(color, vec3(0.9, 0.6, 0.2), f);
        // domain distortion
        a += fbm(st * 20.0) * 0.05;
        // white shards
        f = smoothstep(0.3, 1.0, fbm(vec2(r * 6.0, a * 20.0)));
        color = mix(color, vec3(1), f);
        // dark spots
        f = smoothstep(0.4, 0.9, fbm(vec2(r * 10.0, a * 15.0)));
        color *= 1.0 - f;
        // edge fading
        f = smoothstep(iris_radius, 0.5, r);
        color *= f;
        // pupil
        f = smoothstep(0.2, 0.25, r * pupil_scale);
        color *= f;
        // fake reflection / shine
        f = smoothstep(0.5, 0.0, length(st - shine_cent));
        color += f * vec3(1.0, 0.9, 0.8) * 0.9;
        // edge smoothing
        f = smoothstep(iris_radius - 0.05, iris_radius, r);
        color = mix(color, bg, f);
    }

    r = length(st);
    a = atan(st.y, st.x);

    // corners fade
    f = smoothstep(1.8, 1.0, r);
    color *= f;

    return color;
}

vec3 apply_eye(in vec2 st, in vec3 color, float id, in vec2 shine_cent) {
    if (step(-2.0, st.x) - step(2.0, st.x) == 0.0) {
        return color;
    }

    // get eyelid curves
    float eyelid = pow(cos(st.x * PI * 0.5) * 0.5 + 0.5, 0.5);
    float top_eyelid = smoothstep(0.2, 0.0, eyelid - st.y);
    float bottom_eyelid = smoothstep(0.2, 0.0, eyelid + st.y);
    float eye_mask = top_eyelid + bottom_eyelid;

    if (eye_mask > 0.9) {
        return color;
    }

    color = mix(eye_color(st, vec2(id, 0.0), shine_cent), color, eye_mask);

    float blink = smoothstep(0.05, 0.0, abs(noise3(vec3(time * 0.5, id, 0.0)) - 0.5));
    eyelid = mix(eyelid, 0.0, blink);
    eyelid -= abs(st.y);

    // eye lid shading
    color = mix(color,  vec3(0.15, 0.13, 0.41) * 0.5, smoothstep(0.2, 0.0, eyelid));
    color = mix(color, vec3(0.15, 0.69, 0.58), min(1.0, smoothstep(0.0, -0.1, eyelid)));
    if (eyelid < 0.0) {
        color += min(0.0, eyelid) * 0.3;
        color -= abs(st.x) * 0.1;
    }

    return color;
}

vec3 right_eye(in vec2 st, in vec3 color) {
    st.y -= 0.165;
    st.x += 0.091;
    float angle = -0.2;
    st *= r2(angle);
    st *= 60.0;
    st.y *= 0.8;

    return apply_eye(st, color, 1.0, vec2(-0.35, 0.3));
}

vec3 left_eye(in vec2 st, in vec3 color) {
    st.y -= 0.175;
    st.x -= 0.031;
    float angle = 0.05;
    st *= r2(angle);
    st *= 45.0;
    st.y *= 0.8;

    return apply_eye(st, color, 2.0, vec2(-0.35, 0.3));
}

vec3 third_eye(in vec2 st, in vec3 color) {
    st.y -= 0.233;
    st.x += 0.042;
    float angle = PI * 0.49;
    st *= r2(angle);
    st *= 68.0;
    st.y *= 0.8;

    return apply_eye(st, color, 0.0, vec2(-0.5, -0.3));
}

vec3 apply_eyes(in vec2 st, in vec3 color) {
    color = right_eye(st, color);
    color = left_eye(st, color);
    color = third_eye(st, color);

    return color;
}

// Crystal ball
// ---------------------
// based on Circuits by Kali
// https://www.shadertoy.com/view/XlX3Rj
vec3 formula(in vec2 st, in vec2 c) {
    vec2 z = st;
    float last_stable = 0.0;

    // orbit traps
    float min_comp = 1000.0;
    float min_mag = min_comp;

    float angle = time * 0.05;

    const float iterations = 7;
    for (float i = 0.0; i < iterations; i++) {
        z *= mat2(cos(angle), -sin(angle), sin(angle), cos(angle));
        z = abs(complex_inv(z)) + c;

        float mag = length(z);

        // orbit traps
        float w = 0.1;
        // get minimum component
        float m_comp = clamp(abs(min(z.x, z.y)), w - mag, abs(mag - w));
        // update overall minimum component
        min_comp = min(m_comp, min_comp);
        // update minimum magnitude
        min_mag = min(mag, min_mag);
        // m is 0 unless minimum == min_comp
        // catches the lasst i where z is stable
        last_stable = max(last_stable, i * (1.0 - abs(sign(min_comp - m_comp))));
    }

    last_stable += 1.0;

    float intensity = 0.01;
    float width = intensity * last_stable * 2.0;

    float circ = pow(max(0.0, width - min_mag * 0.1) / width, 6.0);
    float shape = max(pow(max(0.0, width - min_comp) / width, 0.25), circ);

    vec2 m = mouse / resolution.x;

    float t = time * 0.1;
    vec3 color = palette(
        last_stable / iterations,
        vec3(0.5, 0.5, 0.5), 
        vec3(0.5, 0.5, 0.5),
        vec3(1.0, 1.0, 1.0),
        fract(vec3(
            texture(sampler2D(spectrum, spectrum_sampler), vec2(0.7, 0)).x * ball_sensitivity + 0.8 + m.x,
            texture(sampler2D(spectrum, spectrum_sampler), vec2(0.4, 0)).x * ball_sensitivity + 0.1 + m.y,
            texture(sampler2D(spectrum, spectrum_sampler), vec2(0.1, 0)).x * ball_sensitivity
        ))
    );

    // carve out the pattern
    color *= 0.4 + mod(last_stable / iterations + min_mag * 0.2 - t, 1.0) * 1.6;

    return color * shape;
}

vec3 fill_ball(in vec2 st) {
    st *= 10.0;
    
    vec3 color = vec3(0);

    float t = time;
    float scale = 1.5 + sin(t / 25.0);
    st *= scale;

    vec2 c = vec2(-0.4);
    c += vec2(sin(t / 11.0), sin(t / 13.0)) * 0.3;

    color = formula(st, c);

    return color;
}

// Background
// ----------
// based on Random Isometric Blocks by Shane
// https://www.shadertoy.com/view/ltSczW
#define TAU 6.28318530718
const vec2 s = vec2(1, 1.7320508);
float hash21(vec2 p) { return fract(sin(dot(p, vec2(141.13, 289.97))) * 43758.5453); }

// A diamond of sorts - Stretched in a way so as to match the dimensions of a
// cube face in an isometric scene.
float iso_diamond(in vec2 p) {
    p = abs(p);
    return dot(p, s * 0.5); 
}

// This function returns the hexagonal grid coordinate for the grid cell, and the corresponding 
// hexagon cell ID - in the form of the central hexagonal point.
vec4 get_hex(vec2 p) {
    vec4 hC = floor(vec4(p, p - vec2(0.5, 1.0)) / s.xyxy) + 0.5;
    vec4 h = vec4(p - hC.xy * s, p - (hC.zw + 0.5) * s);
    return dot(h.xy, h.xy) < dot(h.zw, h.zw) ? vec4(h.xy, hC.xy) : vec4(h.zw, hC.zw + vec2(0.5, 1));
}

vec3 background(in vec2 st) {
    vec4 hex = get_hex(st * 8.0 + s.yx * time * 0.5);
    vec2 p = hex.xy;

    // Relative squared distance from the center.
    float d = dot(p, p) * 1.5;

    // Using the idetifying coordinate - stored in "h.zw," to produce a unique random number
    // for the hexagonal grid cell.  
    float rnd = hash21(hex.zw);
    float t = rnd * TAU + time * (fract(rnd) + 0.5);
    rnd = sin(t) * 0.5 + 0.5;

    vec3 color = palette(
        rnd,
        vec3(0.5, 0.5, 0.5), 
        vec3(0.5, 0.5, 0.5),
        vec3(1.0, 1.0, 1.0),
        vec3(0.4, 0.1, 1.0)
    );

    // tile flipping
    if (rnd > 0.5) {
        p.xy = -p.xy;
        color *= max(1.25 - d, 0.0);
    } else {
        color *= max(d + 0.55, 0.0);
    }

    // Cube face ID
    float face_id = 0.0;
    if (p.x > 0.0 && -p.y * s.y < p.x * s.x) {
        face_id = 1.0; 
    } else if (p.y * s.y < p.x * s.x) {
        face_id = 2.0;
    }

    color *= mix(0.01, 0.3, clamp(get_spectrum(fract(rnd * face_id + 0.9)), 0.0, 1.0));;

    // Decorating the cube faces:
    // Three rotated diamonds to represent the face borders.
    float di = iso_diamond((p - vec2(0, -0.5) / s)); // bottom
    di = min(di, iso_diamond(r2(TAU / 6.0) * p - vec2(0.0, 0.5) / s)); // left
    di = min(di, iso_diamond(r2(-TAU / 6.0) * p - vec2(0.0, 0.5) / s)); // right
    di -= .25;

    float bord = max(di, -(di + .01));

    // darken inner corners
    if (rnd <= 0.5) {
        color = mix(color, vec3(0), (1.0 - smoothstep(0., .02, bord)) * .7);
    }

    // highlight edge
    color = mix(color, color * 10.0, (1.0 - smoothstep(0., .02, bord - .02)) * .3);
   
    // Cube shading, based on ID. Three different shades for each face of the cube.
    color *= face_id * .5 + .1;

    color *= smoothstep(1.0, 0.5, length(st));

    // Rough gamma correction
    return sqrt(max(color, 0.0));
}

vec2 get_aspect_ratio() {
    return vec2(
        max(1.0, resolution.x / resolution.y),
        max(1.0, resolution.y / resolution.x)
    );
}

void main() {
    vec2 ar = get_aspect_ratio();
    vec2 st = uv * 0.5 * ar;
    vec2 og = st;

    // scale space with bass
    float scale = mix(1.0 + scale_strength, 1.0, get_spectrum(0.1));
    st *= scale;

    // apply random shake with treble
    float shake = mix(0.0, shake_strength, get_spectrum(0.66));
    st += rand2(vec2(0.0, time)) * shake;
    
    // fetch image pixel, if it has color return it
    frag_color = image_color(st, ar);
    // for pictures with drawing
    if (frag_color.a > 0.1) {
        // hack for removing artifact in image
        if (st.x > -0.2 || st.y < 0.2) {
            frag_color = vec4(apply_eyes(st, frag_color.rgb), 1.0);
            return;
        }
    }

    // reset
    frag_color *= 0.0;
	vec3 color = vec3(0.0);

    // crystal ball setup
    const vec2 center = vec2(-0.005, -0.17);
    const float ball_size = 0.25;
    const float ball_border = 0.012;
    const float bg_radius = ball_size + ball_border;
    
    // compute d and masks
    float d = distance(center, st);
    float ball_mask = smoothstep(ball_size, ball_size - 0.001, d);
    float bg_mask = smoothstep(bg_radius, bg_radius + 0.1, d);

    // compute color for pixel
    if (ball_mask > 0.0) {
        color = fill_ball(st - center) * ball_mask * smoothstep(ball_size, ball_size - 0.1, d);
        float shine_d = distance(center + vec2(-0.12, 0.13), st);
        color += pow(smoothstep(0.06, 0.0, shine_d), 4.0) * 0.5;
    } else if (bg_mask > 0.0) {
        color = background(og) * bg_mask;
    }

    frag_color = vec4(color, 1.0);
}
