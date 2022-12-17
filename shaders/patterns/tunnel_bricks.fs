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
            "NAME": "zoom_speed",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 0.5,
            "DEFAULT": 0.05
        },
        {
            "NAME": "rotation_speed",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 0.5,
            "DEFAULT": 0.3
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
            "NAME": "light_x",
            "MIN": -2.0,
            "MAX": 2.0,
            "DEFAULT": 0.0
        },
        {
            "TYPE": "float",
            "NAME": "light_y",
            "MIN": -2.0,
            "MAX": 2.0,
            "DEFAULT": 0.0
        },
        {
            "TYPE": "float",
            "NAME": "light_z",
            "MIN": -20.0,
            "MAX": 0.0,
            "DEFAULT": -1.0
        }
    ]
}*/

#define PI 3.14159265359

const uint max_steps = 256;
const float max_dist = 10.0;
const float surface_dist = 0.0001;
const float ambient = 0.1;

float rand(float n) { return fract(n * 1183.5437 + .42); }

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
    const vec3 lookat = vec3(0.0);

    vec3 forward = normalize(lookat - ro);
    vec3 right = normalize(cross(forward, vec3(0.0, 1.0, 0.0)));
    vec3 up = normalize(cross(right, forward));

    vec3 center = ro + forward * zoom;
    vec3 intersection = center + uv.x * right + uv.y * up;
    vec3 rd = normalize(intersection - ro);

    return rd;
}

float scene_dist(vec3 p) {
    return 1.0 - length(p.xy);
}

vec2 ray_march(vec3 ro, vec3 rd) {
    float dist = 0.0;
    float dist_step = 0.0;
    float layer = -1.0;
    vec3 position;

    for (uint i = 0; i < max_steps; i++) {
        position = ro + rd * dist;
        dist_step = scene_dist(position);
        dist += dist_step * 0.5;

        if (dist >= max_dist || dist_step <= surface_dist) {
            break;
        }
    }

    return vec2(dist, dist_step);
}

vec3 get_normal(vec3 p) {
    vec2 e = vec2(1.0,-1.0)*0.5773;
    const float eps = 0.0005;
    return normalize(e.xyy * scene_dist(p + e.xyy * eps).x + 
					 e.yyx * scene_dist(p + e.yyx * eps).x + 
					 e.yxy * scene_dist(p + e.yxy * eps).x + 
					 e.xxx * scene_dist(p + e.xxx * eps).x);
}

float circle(vec2 st, float r) {
    return smoothstep(r, r + 0.001, length(st));
}

vec3 scene_color(vec3 p, vec3 ro) {
    vec3 normal = get_normal(p);
    if (normal == vec3(0.0)) {
        return normal;
    }

    vec3 light_pos = vec3(light_x, light_y, light_z);
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

    vec2 uv = vec2(atan(p.y, p.x) / PI, p.z);
    vec2 scale = vec2(3.0, 1.0);
    uv *= scale;
    vec2 id = floor(uv);
    uv += vec2(TIME * 0.1, 0.0) * (mod(id.y, 2.0) * 2.0 - 1.0);
    vec2 gv = fract(uv - vec2(0.5, 0.0) * mod(id.y, 2.0));
    id = floor(uv); // - vec2(0.5, 0.0) * mod(id.y, 2.0));

    float edge = (step(0.005, gv.x) - step(0.995, gv.x)) 
        * (step(0.005, gv.y) - step(0.995, gv.y))
        * 0.75 + 0.25;

    for (float x = 0.0; x < 1.0; x += 0.1) {
        edge *= max(circle(gv - vec2(x - 0.05, 0.05), 0.01), 0.5);
        edge *= max(circle(gv - vec2(x + 0.05, 0.95), 0.01), 0.5);
    }

    for (float y = 0.0; y < 1.0; y += 0.1) {
        edge *= max(circle(gv - vec2(0.05, y - 0.05), 0.01), 0.5);
        edge *= max(circle(gv - vec2(0.95, y + 0.05), 0.01), 0.5);
    }

    // vec3 clr = palette(
    //     fract(rand(id.x + id.y) + TIME * color_speed),
    //     vec3(0.5, 0.5, 0.5),
    //     vec3(0.5, 0.5, 0.5),
    //     vec3(1.0, 1.0, 1.0),
    //     color_config.rgb
    // );
    vec3 clr = vec3(0.2);

    return light * clr * edge; // * get_strength(fract(id.x * id.y * 2.0));
}

void main() {
    vec2 st = isf_FragNormCoord - 0.5;
    st.x *= RENDERSIZE.x / RENDERSIZE.y;

    vec3 ro = vec3(0.0, 0.0, 0.1);
    vec3 rd = get_ray_direction(ro, st);
    vec3 p = ro + rd;
    vec2 d = ray_march(ro, rd);

    vec3 color = vec3(0.0);

    if (d.y <= surface_dist) {
        color = scene_color(ro + rd * d.x, ro);
    }

    gl_FragColor = vec4(color, 1);
}
