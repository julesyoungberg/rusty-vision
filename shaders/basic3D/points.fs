/*{
    "DESCRIPTION": "",
    "CREDIT": "by julesyoungberg",
    "ISFVSN": "2.0",
    "CATEGORIES": [ "GENERATOR" ],
    "INPUTS": [
        {
            "TYPE": "float",
            "NAME": "n_steps",
            "MIN": 1.0,
            "MAX": 10.0,
            "DEFAULT": 5.0
        },
        {
            "TYPE": "float",
            "NAME": "spread",
            "MIN": 1.0,
            "MAX": 20.0,
            "DEFAULT": 5.0
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
            "NAME": "light_y",
            "MIN": 0.0,
            "MAX": 20.0,
            "DEFAULT": 10.0
        },
        {
            "TYPE": "float",
            "NAME": "light_z",
            "MIN": -10.0,
            "MAX": 0.0,
            "DEFAULT": -6.0
        },
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
                1.0,
                1.0,
                1.0
            ]
        }
    ]
}*/

const float ambient = 0.1;

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

float line_dist(vec3 ro, vec3 rd, vec3 p) {
    return length(cross(p - ro, rd)) / length(rd);
}

float remap01(float a, float b, float t) { return (t - a) / (b - a); }

vec3 draw_point(vec3 ro, vec3 rd, vec3 p) {
    const float radius = 1.0;

    // float d = line_dist(ro, rd, p);
    // d = smoothstep(radius, radius - 0.01, d);

    float t = dot(p - ro, rd);
    vec3 c = ro + rd * t;

    float y = length(p - c);

    if (y >= radius) {
        return vec3(0.0);
    }

    float x = sqrt(radius * radius - y * y);

    float t1 = t - x;
    vec3 intersection = ro + rd * t1;
    vec3 normal = normalize(intersection - p);

    vec3 light_pos = vec3(0.0, light_y, light_z);
    vec3 light_dir = normalize(light_pos - intersection);

    float diff = max(dot(normal, light_dir), 0.0);
    vec3 diffuse = diff * light_color.rgb;

    return (diffuse + ambient) * color.rgb;
}

void main() {
    vec2 uv = isf_FragNormCoord * 2.0 - 1.0;
    uv.x *= RENDERSIZE.x / RENDERSIZE.y;

    vec3 color = vec3(0.0);

    vec3 ro = vec3(0.0, 0.0, camera_z);
    vec3 rd = get_ray_direction(ro, uv);

    int n = int(floor(n_steps));

    for (int y = -n; y <= n; y++) {
        for (int x = -n; x <= n; x++) {
            vec2 point = vec2(x, y) * spread;
            float z = sin(TIME * speed + length(point) * offset);
            color += draw_point(ro, rd, vec3(point, z));
        }
    }

    gl_FragColor = vec4(color, 1.0);
}
