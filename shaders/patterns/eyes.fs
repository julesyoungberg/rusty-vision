/*{
    "DESCRIPTION": "",
    "CREDIT": "by julesyoungberg",
    "ISFVSN": "2.0",
    "CATEGORIES": [ "GENERATOR" ],
    "INPUTS": []
}*/

// based on Beutypi by iq
// https://www.shadertoy.com/view/lsfGRr

#define PI 3.14159265359
#define AUDIO_REACTIVE 0

const float SCALE = 5.0;
const vec2 s = vec2(1, 1.7320508);

// shane's hexagonal tiling (https://www.shadertoy.com/view/llSyDh)
vec4 get_hex(vec2 p) {
    vec4 hc = floor(vec4(p, p - vec2(0.5, 1)) / s.xyxy) + 0.5;
    vec4 h = vec4(p - hc.xy * s, p - (hc.zw + 0.5) * s);
    return (dot(h.xy, h.xy) < dot(h.zw, h.zw))
               ? vec4(h.xy, hc.xy)
               : vec4(h.zw, hc.zw + vec2(0.5, 1));
}

vec3 hsv2rgb(vec3 c) {
    vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
    return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}

float noise_hash2(vec2 p) {
    p = 50.0 * fract(p * 0.3183099 + vec2(0.71, 0.113));
    return -1.0 + 2.0 * fract(p.x * p.y * (p.x + p.y));
}

float noise_hash3(vec3 p) {
    p = fract(p * 0.3183099 + .1);
    p *= 17.0;
    return fract(p.x * p.y * p.z * (p.x + p.y + p.z));
}

float noise21(in vec2 p) {
    vec2 i = floor(p);
    vec2 f = fract(p);
    vec2 u = f * f * (3.0 - 2.0 * f);

    return mix(mix(noise_hash2(i + vec2(0.0, 0.0)),
                   noise_hash2(i + vec2(1.0, 0.0)), u.x),
               mix(noise_hash2(i + vec2(0.0, 1.0)),
                   noise_hash2(i + vec2(1.0, 1.0)), u.x),
               u.y);
}

float noise31(in vec3 x) {
    vec3 i = floor(x);
    vec3 f = fract(x);
    f = f * f * (3.0 - 2.0 * f);

    return mix(mix(mix(noise_hash3(i + vec3(0, 0, 0)),
                       noise_hash3(i + vec3(1, 0, 0)), f.x),
                   mix(noise_hash3(i + vec3(0, 1, 0)),
                       noise_hash3(i + vec3(1, 1, 0)), f.x),
                   f.y),
               mix(mix(noise_hash3(i + vec3(0, 0, 1)),
                       noise_hash3(i + vec3(1, 0, 1)), f.x),
                   mix(noise_hash3(i + vec3(0, 1, 1)),
                       noise_hash3(i + vec3(1, 1, 1)), f.x),
                   f.y),
               f.z);
}

float rand21(vec2 p) {
    return fract(sin(dot(p.xy, vec2(12.9898, 78.233))) * 43758.5453);
}

float rand31(vec3 p) {
    return fract(sin(dot(p, vec3(27.17, 112.61, 57.53))) * 43758.5453);
}

float fbm(in vec2 p) {
    const mat2 m = mat2(0.8, 0.6, -0.6, 0.8);
    float f = 0.0;
    f += 0.500 * noise21(p);
    p *= m * 2.02;
    f += 0.250 * noise21(p);
    p *= m * 2.03;
    f += 0.125 * noise21(p);
    p *= m * 2.01;
    f += 0.0625 * noise21(p);
    p *= m * 2.04;
    f /= 0.9375;
    return f * 0.5 + 0.5;
}

vec3 eye_color(in vec2 st, in vec2 id, in float closed) {
    st *= 3.5;
    vec3 color = vec3(1);
    vec2 shift = vec2(0);

    float strength = 0.0;
    // if (AUDIO_REACTIVE == 1) {
    //     float i = fract(dot(id, id) * 0.1);
    //     strength = texture(sampler2D(spectrum, spectrum_sampler), vec2(i,
    //     0)).x;
    //     // strength *= (i * 8.0 + 1.0);
    // }

    // follow the mouse if it is pressed, otherwise look around randomly
    // if (mouse_down == 1) {
    //     closed = 0.0;
    //     // get normalized mouse pos
    //     vec2 mouse_pos = 2.0 * mouse / resolution.x;
    //     // get direction to mouse
    //     shift = (id / SCALE - mouse_pos) * 4.0;
    //     shift = vec2(clamp(shift.x, -1.0, 1.0), clamp(shift.y, -0.5, 0.5));
    // } else {
    // get a random shift
    shift = vec2(noise31(vec3(TIME * 2.0, id.x * 17.67, id.y * 23.42)),
                 noise31(vec3(TIME * 2.0, id.x * 11.94, id.y * 27.65))) *
                2.0 -
            1.0;
    shift = pow(shift, vec2(3.0));
    shift *= 0.8;

    // get polar coords
    vec2 ev = st + shift;
    float r = length(ev);
    float a = atan(ev.y, ev.x);

    // animate the radius of the eye
    float ss = sin(TIME * 2.0 + id.x * 3.77 + id.y * 5.33) * 0.1;
    float anim = 1.0 + 0.5 * ss;
    r *= anim;
    // if (AUDIO_REACTIVE == 1) {
    //     r *= smoothstep(0.5, 0.0, strength) * 0.4 + 0.75;
    // }

    // domain distortion
    float a2 = a + fbm(st * 30.0) * 0.05;
    // blood vessels
    float concentration = smoothstep(1.0, 1.8, r) * 0.3;
    float f =
        smoothstep(0.6 - concentration, 1.0, fbm(vec2(r * 6.0, a2 * 50.0)));
    color = mix(color, vec3(1.0, 0.0, 0.0), f);
    vec3 bg = color;

    float iris_radius = 0.9;
    if (r < iris_radius) {
        float pupil_scale =
            (noise31(vec3(TIME * 0.5, id.x * 11.11, id.y * 13.13)) * 0.65 +
             0.4);
        // if (AUDIO_REACTIVE == 1) {
        //     pupil_scale = smoothstep(0.5, 0.05, strength) * 0.5 + 0.5;
        // }
        // eye color
        color = fract(vec3(0.0 + sin(id.x * 3.77), 0.3 + sin(id.y * 5.14),
                           0.4 + sin((id.x + id.y) * 7.35)));
        f = fbm(st * 5.0);
        color = mix(color, vec3(0.2, 0.5, 0.4), f);
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
        f = smoothstep(0.5, 0.0, length(st - vec2(0.24, 0.2)));
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

    // draw eyelids
    float eyelid = pow(cos(st.x * PI * 0.5) * 0.5 + 0.5, 0.5);
    float blink =
        smoothstep(0.05, 0.0, abs(noise31(vec3(TIME * 0.5, id)) - 0.5));
    // if (AUDIO_REACTIVE == 1) {
    //     closed = smoothstep(0.1, 0.09, strength);
    // }
    eyelid = mix(eyelid, 0.0, blink + closed);
    float top_eyelid = smoothstep(0.2, 0.0, eyelid - st.y);
    float bottom_eyelid = smoothstep(0.2, 0.0, eyelid + st.y);
    color = mix(color, vec3(0), top_eyelid + bottom_eyelid);
    return color;
}

void main() {
    vec2 st = isf_FragNormCoord;
    st.y *= RENDERSIZE.y / RENDERSIZE.x;

    // rotate the grid to get a less tiled more natural look
    float angle = tan(s.x / s.y);
    float co = cos(angle);
    float si = sin(angle);
    st *= mat2(co, -si, si, co);

    vec3 color = vec3(0);

    st *= SCALE;

    vec4 coords = get_hex(st);

    vec2 gv = coords.xy;
    vec2 id = coords.zw;

    // reverse rotation so that the eyes are level
    gv *= mat2(co, si, -si, co);
    id *= mat2(co, si, -si, co);
    float closed = smoothstep(0.5, 0.6, noise31(vec3(TIME * 0.2, id * 15.369)));
    color = eye_color(gv, id, closed);

    gl_FragColor = vec4(color, 1);
}
