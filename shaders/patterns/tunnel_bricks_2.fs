/*{
    "DESCRIPTION": "",
    "CREDIT": "by julesyoungberg",
    "ISFVSN": "2.0",
    "CATEGORIES": [ "GENERATOR" ],
    "INPUTS": [
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
            "NAME": "n_layers",
            "TYPE": "float",
            "MIN": 4.0,
            "MAX": 100.0,
            "DEFAULT": 16.0
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
            "MIN": -3.0,
            "MAX": 3.0,
            "DEFAULT": 2.0
        },
        {
            "TYPE": "float",
            "NAME": "light_y",
            "MIN": -3.0,
            "MAX": 3.0,
            "DEFAULT": 2.0
        },
        {
            "TYPE": "float",
            "NAME": "light_z",
            "MIN": -10.0,
            "MAX": 2.0,
            "DEFAULT": -1.0
        },
        {
            "TYPE": "float",
            "NAME": "ambient_strength",
            "MIN": 0.0,
            "MAX": 1.5,
            "DEFAULT": 0.1
        },
        {
            "TYPE": "float",
            "NAME": "diffuse_strength",
            "MIN": 0.0,
            "MAX": 1.5,
            "DEFAULT": 1.0
        },
        {
            "TYPE": "float",
            "NAME": "specular_strength",
            "MIN": 0.0,
            "MAX": 1.5,
            "DEFAULT": 1.0
        },
        {
            "TYPE": "float",
            "NAME": "specular_power",
            "MIN": 0.0,
            "MAX": 256.0,
            "DEFAULT": 32.0
        }
    ]
}*/

#define PI 3.14159265359

const int max_steps = 128;
const float max_dist = 3.0;
const float surface_dist = 0.0001;
const float ambient = 0.1;

float rand(float n) { return fract(n * 1183.5437 + .42); }

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

float sd_cylinder(vec3 p, vec3 a, vec3 b, float r) {
    vec3 ba = b - a;
    vec3 pa = p - a;
    float baba = dot(ba, ba);
    float paba = dot(pa, ba);
    float x = length(pa * baba - ba * paba) - r * baba;
    float y = abs(paba - baba * 0.5) - baba * 0.5;
    float x2 = x * x;
    float y2 = y * y * baba;

    float d = (max(x, y) < 0.0)
        ? -min(x2, y2)
        : (((x > 0.0) ? x2  : 0.0) + ((y > 0.0) ? y2 : 0.0));
    
    return sign(d) * sqrt(abs(d)) / baba;
}

float sd_box(vec3 p, vec3 b) {
    vec3 q = abs(p) - b;
    return length(max(q, 0.0)) + min(max(q.x,max(q.y, q.z)), 0.0);
}

float op_union(float d1, float d2) { return min(d1, d2); }

float op_subtraction(float d1, float d2) { return max(-d1, d2); }

float op_intersection(float d1, float d2) { return max(d1, d2); }

mat3 rot_z(float a) {
    float c = cos(a);
    float s = sin(a);
    return mat3(c, -s, 0.0, s, c, 0.0, 0.0, 0.0, 1.0);
}

float sd_semi_circle(vec3 p, float r1, float r2, float arc) {
    float d1 = sd_cylinder(p, vec3(0.0, 0.0, 0.0), vec3(0.0, 0.0, 0.05), r2);

    float d2 = sd_cylinder(p, vec3(0.0, 0.0, -0.1), vec3(0.0, 0.0, 0.2), r1);

    float d = op_subtraction(d2, d1);

    mat3 r = rot_z(PI * arc);
    vec3 shift = vec3(0.5, 0.0, 0.0);

    float sd = op_intersection(
        sd_box(p - shift, vec3(0.5)),
        sd_box(r * p - shift, vec3(0.5))
    );

    return op_subtraction(sd, d);
}

vec3 scene_dist(vec3 p) {
    vec3 sp = p - vec3(0.0, 0.0, 0.0);
    const float n_layers = 21.0;
    float stp = 1.0 / n_layers;

    float d = max_dist;
    float min_z = 0.0;

    float total_rot = 0.0;

    // p.z *= 0.75;

    for (float z = 0; z < 1.0; z += stp) {
        sp.z = mod(p.z + z - TIME * zoom_speed, 1.0);
        float id = z;
        float arc = round(rand(id * 13.0)) * 0.25 + 0.75;

        // mat3 rot = rot_z(rand(id * 17.0) * PI * 2.0);
        // sp *= rot;

        float layer_rot = TIME * rotation_speed * (rand(id * 23.0) * 2.0 - 1.0);

        mat3 rot = rot_z(layer_rot);

        float dt = sd_semi_circle(sp * rot, 0.11, 0.14, arc);

        if (dt < d) {
            d = dt;
            min_z = sp.z;
            total_rot = layer_rot;
        }
    }

    return vec3(d, min_z, total_rot);
}

vec4 ray_march(vec3 ro, vec3 rd) {
    float dist = 0.0;
    float dist_step = 0.0;
    float layer = -1.0;
    float rot = 0.0;
    vec3 position;

    for (int i = 0; i < max_steps; i++) {
        position = ro + rd * dist;
        vec3 r = scene_dist(position);
        dist_step = r.x;
        layer = r.y;
        rot = r.z;
        dist += dist_step * 0.5;

        if (dist >= max_dist || dist_step <= surface_dist) {
            break;
        }
    }

    return vec4(dist, dist_step, layer, rot);
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

vec3 scene_color(in vec3 p, vec3 ro, float z, float rot) {
    vec3 normal = get_normal(p);
    if (normal == vec3(0.0)) {
        return normal;
    }

    p *= rot_z(rot);
    p.z = z;

    vec3 light_pos = vec3(light_x, light_y, light_z);
    vec3 light_dir = normalize(light_pos - p);

    float diff = max(dot(normal, light_dir), 0.0);
    float diffuse = diffuse_strength * diff;

    vec3 view_dir = normalize(ro - p);
    vec3 reflect_dir = reflect(-light_dir, normal);

    float spec = pow(max(dot(view_dir, reflect_dir), 0.0), specular_power);
    float specular = specular_strength * spec;

    vec3 light = vec3(diffuse + ambient_strength + specular);

    vec2 uv = vec2(atan(p.y, p.x) / PI, p.z);
    vec2 scale = vec2(4.0, 4.0);
    uv *= scale;

    vec2 id = floor(uv);
    vec2 gv = fract(uv - vec2(0.5, 0.0) * mod(id.y, 2.0));
    id = floor(uv); // - vec2(0.5, 0.0) * mod(id.y, 2.0));

    float edge = (step(0.005, gv.x) - step(0.995, gv.x))
        * (step(0.005, gv.y) - step(0.995, gv.y))
        * 0.75 + 0.25;

    const vec2 correction = vec2(1.0, 2.0);

    gv *= correction;

    for (float x = 0.0; x < 1.0; x += 0.1) {
        edge *= max(circle(gv - vec2(x - 0.05, 0.05), 0.01), 0.5);
        edge *= max(circle(gv - vec2(x + 0.05, 0.35), 0.01), 0.5);
    }

    for (float y = 0.0; y < 0.5; y += 0.05) {
        edge *= max(circle(gv - vec2(0.05, y - 0.05), 0.01), 0.5);
        edge *= max(circle(gv - vec2(0.95, y + 0.05), 0.01), 0.5);
    }

    vec3 clr = vec3(0.2);

    return light * clr * edge;
}

void main() {
    vec2 st = isf_FragNormCoord - 0.5;
    st.x *= RENDERSIZE.x / RENDERSIZE.y;

    vec3 ro = vec3(0.0, 0.0, 0.1);
    vec3 rd = get_ray_direction(ro, st);
    vec4 d = ray_march(ro, rd);

    vec3 color = vec3(0.0);

    if (d.y <= surface_dist) {
        color = scene_color(ro + rd * d.x, ro, d.z, d.a);
    }

    gl_FragColor = vec4(color, 1);
}
