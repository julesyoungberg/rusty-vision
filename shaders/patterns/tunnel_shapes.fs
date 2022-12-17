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
            "DEFAULT": 0.1
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
            "DEFAULT": 1.0
        },
        {
            "TYPE": "float",
            "NAME": "light_y",
            "MIN": -2.0,
            "MAX": 2.0,
            "DEFAULT": 1.0
        },
        {
            "TYPE": "float",
            "NAME": "light_z",
            "MIN": -10.0,
            "MAX": 2.0,
            "DEFAULT": -1.0
        }
    ]
}*/

#define PI 3.14159265359

const uint max_steps = 128;
const float max_dist = 3.0;
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

float sd_box(vec3 p, vec3 b) {
    vec3 q = abs(p) - b;
    return length(max(q, 0.0)) + min(max(q.x,max(q.y, q.z)), 0.0);
}

float sd_box_frame(vec3 p, vec3 b, float e) {
    p = abs(p) - b;
    vec3 q = abs(p + e) - e;
    return min(min(
        length(max(vec3(p.x,q.y,q.z),0.0))+min(max(p.x,max(q.y,q.z)),0.0),
        length(max(vec3(q.x,p.y,q.z),0.0))+min(max(q.x,max(p.y,q.z)),0.0)),
        length(max(vec3(q.x,q.y,p.z),0.0))+min(max(q.x,max(q.y,p.z)),0.0));
}


float sd_octahedron(vec3 p, float s) {
    p = abs(p);
    return (p.x + p.y + p.z - s) * 0.57735027;
}

float sd_pyramid(vec3 p, float h) {
    float m2 = h*h + 0.25;
        
    p.xz = abs(p.xz);
    p.xz = (p.z>p.x) ? p.zx : p.xz;
    p.xz -= 0.5;

    vec3 q = vec3( p.z, h*p.y - 0.5*p.x, h*p.x + 0.5*p.y);
    
    float s = max(-q.x,0.0);
    float t = clamp( (q.y-0.5*p.z)/(m2+0.25), 0.0, 1.0 );
        
    float a = m2*(q.x+s)*(q.x+s) + q.y*q.y;
    float b = m2*(q.x+0.5*t)*(q.x+0.5*t) + (q.y-m2*t)*(q.y-m2*t);
        
    float d2 = min(q.y,-q.x*m2-q.y*0.5) > 0.0 ? 0.0 : min(a,b);
        
    return sqrt( (d2+q.z*q.z)/m2 ) * sign(max(q.z,-p.y));
}

float op_union(float d1, float d2) { return min(d1, d2); }

float op_subtraction(float d1, float d2) { return max(-d1, d2); }

mat3 rot_z(float a) {
    float c = cos(a);
    float s = sin(a);
    return mat3(c, -s, 0.0, s, c, 0.0, 0.0, 0.0, 1.0);
}

vec2 scene_dist(vec3 p) {
    vec3 sp = p - vec3(0.0, 0.0, 0.0);
    const float n_layers = 24.0;
    float stp = 1.0 / n_layers;

    float d = max_dist;
    float min_z = 0.0;

    for (float z = 0; z < 1.0; z += stp) {
        sp.z = mod(p.z + z - TIME * zoom_speed, 1.0);
        // float arc = rand(id * 13.0) * 0.8 + 0.1;

        vec3 tp = sp;

        // if (rand(z * 29.0) > 0.5) {
        //     tp *= rot_z(rand(z * 17.0) * PI * 2.0);
        // }

        tp.xy = abs(tp.xy);

        if (rand(z * n_layers) > 0.4) {
            tp.x *= tp.x;
            tp.y *= tp.y;
        }

        // float a = atan(tp.y, tp.x);
        // float r = length(tp.xy);

        // tp.x = cos(a) * r;
        // tp.y = sin(a) * r;

        tp.xy -= vec2(rand(z * 7.0) * 0.1 + 0.13);

        tp *= rot_z(TIME * rotation_speed * (rand(z * 23.0) * 2.0 - 1.0));

        vec3 size = vec3(
            rand(z * 13.0) * 0.075 + 0.05,
            rand(z * 17.0) * 0.075 + 0.05,
            stp
        );

        float shape = rand(z * 23.0);

        float dt = max_dist;

        if (shape < 0.33) {
            dt = sd_box(tp, size);
        } else if (shape < 0.66) {
            dt = sd_box_frame(tp, size, 0.01);
        } else {
            dt = sd_octahedron(tp, size.x);
        }

        if (dt < d) {
            d = dt;
            min_z = z;
        }
    }

    return vec2(d, min_z);
}

vec3 ray_march(vec3 ro, vec3 rd) {
    float dist = 0.0;
    float dist_step = 0.0;
    float layer = -1.0;
    vec3 position;

    for (uint i = 0; i < max_steps; i++) {
        position = ro + rd * dist;
        vec2 r = scene_dist(position);
        dist_step = r.x;
        layer = r.y;
        dist += dist_step * 0.5;

        if (dist >= max_dist || dist_step <= surface_dist) {
            break;
        }
    }

    return vec3(dist, dist_step, layer);
}

vec3 get_normal(vec3 p) {
    vec2 e = vec2(1.0,-1.0)*0.5773;
    const float eps = 0.0005;
    return normalize(e.xyy * scene_dist(p + e.xyy * eps).x + 
					 e.yyx * scene_dist(p + e.yyx * eps).x + 
					 e.yxy * scene_dist(p + e.yxy * eps).x + 
					 e.xxx * scene_dist(p + e.xxx * eps).x);
}

vec3 scene_color(vec3 p, vec3 ro, float id) {
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

    vec3 clr = palette(
        fract(id * 3.0 + TIME * color_speed),
        vec3(0.5, 0.5, 0.5),
        vec3(0.5, 0.5, 0.5),
        vec3(1.0, 1.0, 1.0),
        color_config.rgb
    );

    return light * clr * get_strength(fract(id * 2.0));
}

void main() {
    vec2 st = isf_FragNormCoord - 0.5;
    st.x *= RENDERSIZE.x / RENDERSIZE.y;

    vec3 ro = vec3(0.0, 0.0, 0.1);
    vec3 rd = get_ray_direction(ro, st);
    vec3 d = ray_march(ro, rd);

    vec3 color = vec3(0.0);

    if (d.y <= surface_dist) {
        color = scene_color(ro + rd * d.x, ro, d.z);
    }

    gl_FragColor = vec4(color, 1);
}
