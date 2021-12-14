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
            "NAME": "scale",
            "TYPE": "float",
            "MIN": 1.0,
            "MAX": 50.0,
            "DEFAULT": 16.0
        },
        {
            "NAME": "speed",
            "TYPE": "float",
            "MIN": -2.0,
            "MAX": 2.0,
            "DEFAULT": 1.0
        },
        {
            "NAME": "point_speed",
            "TYPE": "float",
            "MIN": -0.5,
            "MAX": 0.5,
            "DEFAULT": 0.5
        },
        {
            "NAME": "sensitivity",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 20.0,
            "DEFAULT": 10.0
        },
        {
            "NAME": "color_palette",
            "TYPE": "color",
            "DEFAULT": [
                1.0,
                0.9,
                0.5,
                1.0
            ]
        }
    ]
}*/

#define TAU 6.28318530718

vec3 hsv2rgb(vec3 c) {
    vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
    return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}

// IQ's palette generator:
// https://www.iquilezles.org/www/articles/palettes/palettes.htm
vec3 palette(in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d) {
    return a + b * cos(6.28318 * (c * t + d));
}

vec2 rand2(vec2 p) {
    return fract(
        sin(vec2(dot(p, vec2(127.1, 311.7)), dot(p, vec2(269.5, 183.3)))) *
        43758.5453);
}

const vec2 s = vec2(1, 1.7320508);

// shane's hexagonal tiling (https://www.shadertoy.com/view/llSyDh)
vec4 get_hex(vec2 p) {
    vec4 hc = floor(vec4(p, p - vec2(0.5, 1)) / s.xyxy) + 0.5;
    vec4 h = vec4(p - hc.xy * s, p - (hc.zw + 0.5) * s);
    return (dot(h.xy, h.xy) < dot(h.zw, h.zw))
               ? vec4(h.xy, hc.xy)
               : vec4(h.zw, hc.zw + vec2(0.5, 1));
}

float hex_dist(in vec2 p) {
    p = abs(p);
    return max(dot(p, normalize(s)), p.x);
}

vec2 get_point(vec2 coord) {
    vec2 point = rand2(coord);
    return vec2(cos(TIME * point_speed + point.x * TAU),
                sin(TIME * point_speed + point.y * TAU)) *
           0.3;
}

vec3 voronoi(vec4 coords, vec2 st, float scale) {
    vec2 gv = coords.xy;
    vec2 id = coords.zw;

    float m_dist = scale;
    vec2 m_point;
    vec2 m_coord;
    vec2 m_diff;

// find the nearest cell center
#pragma unroll
    for (float i = 0.0; i <= 6.0; i++) {
        vec2 offset = vec2(0);
        if (i < 6.0) {
            float angle = i * TAU / 6.0;
            float si = sin(angle);
            float co = cos(angle);
            offset = vec2(1.0, 0.0) * mat2(co, -si, si, co);
        }

        vec2 coord = get_hex(st + offset).zw;
        vec2 point = get_point(coord);

        vec2 diff = offset + point - gv;
        float dist = length(diff);

        if (dist < m_dist) {
            m_dist = dist;
            m_point = point;
            m_coord = coord;
            m_diff = diff;
        }
    }

    float m_edge_dist = scale;

// find the nearest edge
#pragma unroll
    for (float i = 0.0; i <= 6.0; i++) {
        vec2 offset = vec2(0);
        if (i < 6.0) {
            float angle = i * TAU / 6.0;
            float si = sin(angle);
            float co = cos(angle);
            offset = vec2(1.0, 0.0) * mat2(co, -si, si, co);
        }

        vec2 coord = get_hex(st + offset).zw;
        if (all(equal(m_coord, coord))) {
            continue;
        }

        vec2 point = get_point(coord);

        vec2 diff = offset + point - gv;
        float dist = length(diff);

        vec2 to_center = (m_diff + diff) * 0.5;
        vec2 cell_diff = normalize(diff - m_diff);
        float edge_dist = dot(to_center, cell_diff);
        m_edge_dist = min(m_edge_dist, edge_dist);
    }

    return vec3(m_point, m_edge_dist);
}

//  Function from Iñigo Quiles
// https://www.iquilezles.org/www/articles/functions/functions.htm
float impulse(float x, float k) {
    float h = k * x;
    return h * exp(1.0 - h);
}

void main() {
    vec2 st = isf_FragNormCoord * 2.0 - 1.0;
    st.y *= RENDERSIZE.y / RENDERSIZE.x;

    vec3 color = vec3(0);

    float r = length(st);
    float scaling = 1.0;
    if (r < 0.5) {
        scaling = pow(smoothstep(0.0, 0.5, r) * 2.0, 4.0);
    } else {
        scaling = scale - smoothstep(0.5, 0.75, r) * 2.0;
    }
    st *= scaling;
    st += TIME * speed;

    float scale = 1.0;
    st *= scale;

    vec4 coords = get_hex(st);
    vec3 val = voronoi(coords, st, scale);
    vec2 m_point = val.xy;
    float m_edge_dist = val.z;

    // color = hsv2rgb(vec3(fract(dot(m_point, m_point)), 1, 1));
    // color = mix(vec3(0), color, smoothstep(0.01, 0.02, m_edge_dist));
    // // color += scaling;
    // float radius = 1.0;
    // color += smoothstep(radius, radius + 0.01, r) - smoothstep(radius + 0.01,
    // radius + 0.02, r);

    // map point to 1d value between 0 and 1
    float point_val = fract(dot(m_point, m_point) * 4.38);
    float intensity =
        log(IMG_NORM_PIXEL(fft_texture, vec2(point_val, 0)).x + 1.0);

    color = palette(point_val, vec3(0.5, 0.5, 0.5), vec3(0.5, 0.5, 0.5),
                    vec3(1.0, 1.0, 1.0), color_palette.rgb) *
            log(intensity * sensitivity);
    color = mix(vec3(0), color, smoothstep(0.05, 0.06, m_edge_dist));

    // dots
    // color += 1.0 - step(0.02, m_dist);
    // grid
    // color.r += step(0.48, hex_dist(gv));

    gl_FragColor = vec4(color, 1);
}
