/*{
    "DESCRIPTION": "",
    "CREDIT": "by julesyoungberg",
    "ISFVSN": "2.0",
    "CATEGORIES": [ "GENERATOR" ],
    "INPUTS": [
        {
            "NAME": "fft_texture",
            "TYPE": "audioFFT"
        },
        {
            "NAME": "color_config",
            "TYPE": "color",
            "DEFAULT": [
                0.60,
                0.10,
                0.20,
                1.0
            ]
        },
        {
            "NAME": "sensitivity",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 1.0,
            "DEFAULT": 0.5
        },
        {
            "NAME": "brightness",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 1.0,
            "DEFAULT": 0.5
        },
        {
            "NAME": "look_x",
            "TYPE": "float",
            "MIN": -5.0,
            "MAX": 5.0,
            "DEFAULT": 3.0
        },
        {
            "NAME": "look_z",
            "TYPE": "float",
            "MIN": -5.0,
            "MAX": 5.0,
            "DEFAULT": 3.0
        },
        {
            "NAME": "zoom_speed",
            "TYPE": "float",
            "MIN": -2.0,
            "MAX": 2.0,
            "DEFAULT": -1.0
        },
        {
            "NAME": "noise_speed",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 1.0,
            "DEFAULT": 0.2
        },
        {
            "NAME": "color_speed",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 1.0,
            "DEFAULT": 0.1
        },
        {
            "NAME": "n_layers",
            "TYPE": "float",
            "MIN": 4.0,
            "MAX": 100.0,
            "DEFAULT": 32.0
        },
        {
            "NAME": "color_amount",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 1.0,
            "DEFAULT": 1.0
        },
        {
            "TYPE": "float",
            "NAME": "zoom",
            "MIN": 0.5,
            "MAX": 2.0,
            "DEFAULT": 0.5
        },
        {
            "TYPE": "float",
            "NAME": "fog_amount",
            "MIN": 0.0,
            "MAX": 3.0,
            "DEFAULT": 1.1
        },
        {
            "TYPE": "float",
            "NAME": "noise_amount",
            "MIN": 0.0,
            "MAX": 20.0,
            "DEFAULT": 10.0
        },
        {
            "TYPE": "float",
            "NAME": "noise_scale",
            "MIN": 0.0,
            "MAX": 1.0,
            "DEFAULT": 0.1
        },
        {
            "TYPE": "float",
            "NAME": "grid_scale",
            "MIN": 0.0,
            "MAX": 2.0,
            "DEFAULT": 1.0
        },
        {
            "TYPE": "float",
            "NAME": "light_x",
            "MIN": -10.0,
            "MAX": 10.0,
            "DEFAULT": -5.0
        },
        {
            "TYPE": "float",
            "NAME": "light_y",
            "MIN": -2.0,
            "MAX": 10.0,
            "DEFAULT": -7.0
        },
        {
            "TYPE": "float",
            "NAME": "light_z",
            "MIN": -10.0,
            "MAX": 10.0,
            "DEFAULT": -8.0
        }
    ]
}*/

#define PI 3.14159265359

const uint max_steps = 256;
const float max_dist = 200.0;
const float surface_dist = 0.0001;
const float ambient = 0.1;

float rand(float n) { return fract(n * 1183.5437 + .42); }

float rand21(vec2 p) {
    return fract(sin(dot(p.xy, vec2(12.9898, 78.233))) * 43758.5453);
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

// IQ's palette generator:
// https://www.iquilezles.org/www/articles/palettes/palettes.htm
vec3 palette(in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d) {
    return a + b * cos(6.28318 * (c * t + d));
}

float get_strength(float i) {
    return mix(
        brightness,
        log(IMG_NORM_PIXEL(fft_texture, vec2(i, 0)).x + 1.0),
        sensitivity
    );
}

vec3 get_ray_direction(vec3 ro, vec2 uv) {
    const vec3 lookat = ro - vec3(look_x, 1.0, look_z);

    vec3 forward = normalize(lookat - ro);
    vec3 right = normalize(cross(forward, vec3(0.0, 1.0, 0.0)));
    vec3 up = normalize(cross(right, forward));

    vec3 center = ro + forward * zoom;
    vec3 intersection = center + uv.x * right + uv.y * up;
    vec3 rd = normalize(intersection - ro);

    return rd;
}

float sd_box(vec3 p, vec3 b) {
    vec3 q = abs(p) - b;
    return length(max(q, 0.0)) + min(max(q.x,max(q.y, q.z)), 0.0);
}

float op_union(float d1, float d2) { return min(d1, d2); }

float op_subtraction(float d1, float d2) { return max(-d1, d2); }

vec2 get_cell(vec3 p) {
    return floor(p.xz * grid_scale);
}

float scene_dist(vec3 p) {
    p.y = -abs(p.y);

    vec2 cell = get_cell(p);
    float t = TIME * noise_speed;
    float height_offset = -2.0 - noise31(vec3(cell * noise_scale, t)) * noise_amount;
    float plane = dot(p, normalize(vec3(0.0, 1.0, 0.0))) - height_offset;

    float min_dist = plane;

    // for (float z = -1.0; z <= 1.0; z += 1.0) {
    //     for (float x = -1.0; x <= 1.0; x += 1.0) {
    //         vec2 c = cell + vec2(x, z);
    //         float height_offset = -2.0 - noise31(vec3(cell * noise_scale, t)) * noise_amount;
    //         float d = sd_box(p - vec3(c.x, height_offset, c.y), vec3(1.0));

    //         if (abs(d) < abs(min_dist)) {
    //             min_dist = d;
    //         }
    //     }
    // }

    return min_dist * 0.5;
}

vec2 ray_march(vec3 ro, vec3 rd) {
    float dist = 0.0;
    float dist_step = 0.0;

    for (uint i = 0; i < max_steps; i++) {
        dist_step = scene_dist(ro + rd * dist);
        dist += dist_step;

        if (dist >= max_dist || abs(dist_step) <= surface_dist) {
            break;
        }
    }

    return vec2(dist, dist_step);
}

vec3 get_normal(vec3 p) {
    vec2 e = vec2(1.0,-1.0)*0.5773;
    const float eps = 0.0005;
    return normalize(e.xyy * scene_dist(p + e.xyy * eps) + 
					 e.yyx * scene_dist(p + e.yyx * eps) + 
					 e.yxy * scene_dist(p + e.yxy * eps) + 
					 e.xxx * scene_dist(p + e.xxx * eps));
}

vec3 scene_color(vec3 p, vec3 ro) {
    vec3 normal = get_normal(p);
    if (normal == vec3(0.0)) {
        return normal;
    }

    vec3 light_pos = ro + vec3(light_x, light_y, light_z);
    vec3 light_dir = normalize(light_pos - p);

    const float ambient = 0.1;
    const float diffuse_strength = 1.0;
    const float specular_strength = 1.0;

    float diff = max(dot(normal, light_dir), 0.0);
    float diffuse = diffuse_strength * diff;

    vec3 view_dir = normalize(ro - p);
    vec3 reflect_dir = reflect(-light_dir, normal);

    float spec_pow = 32.0;
    float spec = pow(max(dot(view_dir, reflect_dir), 0.0), spec_pow);
    float specular = specular_strength * spec;

    vec3 light = vec3(diffuse + ambient + specular);

    float id = rand21(get_cell(p));

    vec3 clr = palette(
        fract(id * 5.0 + TIME * color_speed),
        vec3(0.5, 0.5, 0.5),
        vec3(0.5, 0.5, 0.5),
        vec3(1.0, 1.0, 1.0),
        color_config.rgb
    ) * color_amount * get_strength(fract(id * 0.1));

    return light * clr;
}

void main() {
    vec2 st = isf_FragNormCoord - 0.5;
    st.x *= RENDERSIZE.x / RENDERSIZE.y;

    st.xy = st.yx;
    st.y *= st.y;

    float t = TIME * zoom_speed;
    vec3 ro = vec3(t, 0.0, t);
    vec3 rd = get_ray_direction(ro, st);
    vec2 d = ray_march(ro, rd);

    vec3 color = vec3(0.0);

    if (abs(d.y) <= surface_dist) {
        color = scene_color(ro + rd * d.x, ro);
    }

    color = mix(color, vec3(0.8), d.x / max_dist * fog_amount);

    gl_FragColor = vec4(color, 1);
}
