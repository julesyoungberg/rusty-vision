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
            "DEFAULT": 20.0
        },
        {
            "NAME": "speed",
            "TYPE": "float",
            "MIN": -1.0,
            "MAX": 1.0,
            "DEFAULT": 0.5
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

// IQ's palette generator:
// https://www.iquilezles.org/www/articles/palettes/palettes.htm
vec3 palette(in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d) {
    return a + b * cos(6.28318 * (c * t + d));
}

vec3 rand3(vec3 p) {
    return fract(sin(vec3(dot(p, vec3(127.1, 311.7, 264.9)),
                          dot(p, vec3(269.5, 183.3, 491.5)),
                          dot(p, vec3(27.17, 112.61, 57.53)))) *
                 43758.5453);
}

vec3 get_point(vec3 coord) {
    vec3 point = rand3(coord);
    point = sin(TIME * point_speed + 6.2831 * point) * 0.5 + 0.5;
    return point;
}

vec4 voronoi(vec3 p, float scale) {
    vec3 i_st = floor(p);
    vec3 f_st = fract(p);

    float m_dist = scale;
    vec3 m_point;
    vec3 m_coord;
    vec3 m_diff;

    // find the nearest cell center
    for (int z = -1; z <= 1; z++) {
        for (int y = -1; y <= 1; y++) {
            for (int x = -1; x <= 1; x++) {
                vec3 neighbor = vec3(x, y, z);
                vec3 coord = i_st + neighbor;
                vec3 point = get_point(coord);

                vec3 diff = neighbor + point - f_st;
                float dist = length(diff);

                if (dist < m_dist) {
                    m_dist = dist;
                    m_point = point;
                    m_coord = coord;
                    m_diff = diff;
                }
            }
        }
    }

    float m_edge_dist = scale;

    // find the nearest edge
    for (int z = -1; z <= 1; z++) {
        for (int y = -1; y <= 1; y++) {
            for (int x = -1; x <= 1; x++) {
                vec3 neighbor = vec3(x, y, z);
                vec3 coord = i_st + neighbor;
                if (all(equal(m_coord, coord))) {
                    continue;
                }

                vec3 point = get_point(coord);

                vec3 diff = neighbor + point - f_st;
                float dist = length(diff);

                vec3 to_center = (m_diff + diff) * 0.5;
                vec3 cell_diff = normalize(diff - m_diff);
                float edge_dist = dot(to_center, cell_diff);
                m_edge_dist = min(m_edge_dist, edge_dist);
            }
        }
    }

    return vec4(m_point, m_edge_dist);
}

void main() {
    vec2 st = isf_FragNormCoord * 2.0 - 1.0;
    st.y *= RENDERSIZE.y / RENDERSIZE.x;
    st = st * 0.5 + 0.5;

    st *= scale;

    vec3 p = vec3(st, TIME * speed);
    vec4 val = voronoi(p, scale);
    vec3 m_point = val.xyz;
    float m_edge_dist = val.w;

    // map point to 1d value between 0 and 1
    float point_val = dot(m_point, m_point) * 0.5;
    float intensity =
        log(IMG_NORM_PIXEL(fft_texture, vec2(point_val, 0)).x + 1.0);

    vec3 color = palette(point_val, vec3(0.5, 0.5, 0.5), vec3(0.5, 0.5, 0.5),
                         vec3(1.0, 1.0, 1.0), color_palette.rgb) *
                 log(intensity * sensitivity);
    color = mix(vec3(0), color, smoothstep(0.05, 0.06, m_edge_dist));

    // Draw cell center
    // color += 1.-step(.02, m_dist);

    // Draw grid
    // color.r += step(.98, f_st.x) + step(.98, f_st.y);

    gl_FragColor = vec4(max(vec3(0), color), 1.0);
}
