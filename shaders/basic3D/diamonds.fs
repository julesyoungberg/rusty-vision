/*{
    "DESCRIPTION": "",
    "CREDIT": "by julesyoungberg",
    "ISFVSN": "2.0",
    "CATEGORIES": [ "GENERATOR" ],
    "INPUTS": [
        {
            "NAME": "light_color",
            "TYPE": "color",
            "DEFAULT": [
                1.0,
                1.0,
                1.0,
                1.0
            ]
        },
        {
            "NAME": "color",
            "TYPE": "color",
            "DEFAULT": [
                1.0,
                0.1,
                1.0,
                1.0
            ]
        },
        {
            "TYPE": "float",
            "NAME": "n_steps",
            "MIN": 0.0,
            "MAX": 10.0,
            "DEFAULT": 2.0
        },
        {
            "TYPE": "float",
            "NAME": "spread",
            "MIN": 13.0,
            "MAX": 30.0,
            "DEFAULT": 13.0
        },
        {
            "TYPE": "float",
            "NAME": "speed",
            "MIN": 0.0,
            "MAX": 3.0,
            "DEFAULT": 1.5
        },
        {
            "TYPE": "float",
            "NAME": "offset",
            "MIN": 0.0,
            "MAX": 1.0,
            "DEFAULT": 0.2
        },
        {
            "TYPE": "float",
            "NAME": "camera_z",
            "MIN": -30.0,
            "MAX": -5.0,
            "DEFAULT": -20.0
        },
        {
            "TYPE": "float",
            "NAME": "zoom",
            "MIN": 0.5,
            "MAX": 2.0,
            "DEFAULT": 1.0
        },
        {
            "TYPE": "float",
            "NAME": "light_x",
            "MIN": 0.0,
            "MAX": 20.0,
            "DEFAULT": 2.0
        },
        {
            "TYPE": "float",
            "NAME": "light_y",
            "MIN": 0.0,
            "MAX": 20.0,
            "DEFAULT": 2.0
        },
        {
            "TYPE": "float",
            "NAME": "light_z",
            "MIN": -10.0,
            "MAX": 0.0,
            "DEFAULT": -5.0
        }
    ]
}*/

const uint max_steps = 100;
const float max_dist = 100.0;
const float surface_dist = 0.1;

vec3 get_ray_direction(vec3 ro, vec2 uv) {
    const vec3 lookat = vec3(0.0);

    vec3 forward = normalize(lookat - ro);
    vec3 right = cross(vec3(0.0, 1.0, 0.0), forward);
    vec3 up = cross(forward, right);

    vec3 center = ro + forward * zoom;
    vec3 intersection = center + uv.x * right + uv.y * up;
    vec3 rd = normalize(intersection - ro);

    return rd;
}

// https://www.iquilezles.org/www/articles/distfunctions/distfunctions.htm
float octahedron(vec3 p, float s) {
    p = abs(p);
    float m = p.x + p.y + p.z - s;
    vec3 q;
    if (3.0 * p.x < m)
        q = p.xyz;
    else if (3.0 * p.y < m)
        q = p.yzx;
    else if (3.0 * p.z < m)
        q = p.zxy;
    else
        return m * 0.57735027;

    float k = clamp(0.5 * (q.z - q.y + s), 0.0, s);
    return length(vec3(q.x, q.y - s + k, q.z - k));
}

vec3 op_twist(in vec3 p, float k) {
    float c = cos(k * p.y);
    float s = sin(k * p.y);
    mat2 m = mat2(c, -s, s, c);
    vec3 q = vec3(m * p.xz, p.y);
    return q;
}

mat4 rotate_y(in float angle) {
    return mat4(cos(angle), 0, sin(angle), 0, 0, 1.0, 0, 0, -sin(angle), 0,
                cos(angle), 0, 0, 0, 0, 1);
}

float diamond(vec3 p, float offst) {
    mat4 rotation = rotate_y(TIME * speed);
    vec4 rotated = rotation * vec4(p, 1.0);
    return octahedron(op_twist(rotated.xyz, sin(TIME * speed + offst)), 6.0);
}

float scene_dist(vec3 p) {
    int n = int(floor(n_steps));
    float min_dist = max_dist * 2.0;
    vec3 offst = vec3(0.0);
    vec3 point = vec3(0.0);

    for (int y = -n; y <= n; y++) {
        for (int x = -n; x <= n; x++) {
            offst = vec3(x, y, 0.0) * spread;
            point = p + offst;
            min_dist = min(diamond(point, length(offst) * offset), min_dist);
        }
    }

    return min_dist;
}

float ray_march(vec3 ro, vec3 rd) {
    float dist = 0.0;
    float dist_step = 0.0;
    vec3 position;

    for (uint i = 0; i < max_steps; i++) {
        position = ro + rd * dist;
        dist_step = scene_dist(position);
        dist += dist_step * 0.5;

        if (dist >= max_dist || dist_step <= surface_dist) {
            break;
        }
    }

    return dist;
}

vec3 get_normal(vec3 p) {
    const vec2 e = vec2(0.1, 0.0);
    float d = scene_dist(p);

    if (d > surface_dist) {
        return vec3(0.0);
    }

    vec3 n = d - vec3(scene_dist(p - e.xyy), scene_dist(p - e.yxy),
                      scene_dist(p - e.yyx));

    return normalize(n);
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

    float spec_pow = 8.0;
    float spec = pow(max(dot(view_dir, reflect_dir), 0.0), spec_pow);
    float specular = specular_strength * spec;

    vec3 light = (diffuse + ambient + specular) * light_color.rgb;

    return light * color.rgb * (normal + 1.0);
}

void main() {
    vec2 uv = isf_FragNormCoord * 2.0 - 1.0;
    uv.x *= RENDERSIZE.x / RENDERSIZE.y;

    vec3 ro = vec3(0.0, 0.0, camera_z);
    vec3 rd = get_ray_direction(ro, uv);
    float dist = ray_march(ro, rd);

    vec3 color = scene_color(ro + rd * dist, ro);

    gl_FragColor = vec4(color, 1.0);
}
