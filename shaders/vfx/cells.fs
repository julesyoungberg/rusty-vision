/*{
    "DESCRIPTION": "Voronoi cell effect.",
    "CREDIT": "by julesyoungberg",
    "ISFVSN": "2.0",
    "CATEGORIES": [ "Blur" ],
    "INPUTS": [
        {
            "NAME": "inputImage",
            "TYPE": "image"
        },
        {
            "NAME": "speed",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 1.0,
            "DEFAULT": 0.4
        },
        {
            "NAME": "scale",
            "TYPE": "float",
            "MIN": 1.0,
            "MAX": 50.0,
            "DEFAULT": 20.0
        },
        {
            "NAME": "edge_thickenss",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 0.1,
            "DEFAULT": 0.06
        }
    ]
}*/

vec3 rand3(vec3 p) {
    return fract(sin(vec3(dot(p, vec3(127.1, 311.7, 264.9)),
                          dot(p, vec3(269.5, 183.3, 491.5)),
                          dot(p, vec3(27.17, 112.61, 57.53)))) *
                 43758.5453);
}

vec3 hsv2rgb(vec3 c) {
    vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
    return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}

vec3 image_color(in vec2 coord) {
    return IMG_NORM_PIXEL(inputImage, fract(coord)).rgb;
}

vec3 get_point(vec3 coord) {
    vec3 point = rand3(coord);
    point = sin(TIME * 0.2 + 6.2831 * point) * 0.5 + 0.5;
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

    return vec4(m_coord + m_point, m_edge_dist);
}

void main() {
    vec2 st = isf_FragNormCoord;

    st *= scale;

    vec3 p = vec3(st, TIME * speed);
    vec4 val = voronoi(p, scale);
    vec3 m_point = val.xyz;
    float m_edge_dist = val.w;

    vec2 g_point = m_point.xy;
    vec2 coord = g_point / scale;
    vec3 color = image_color(coord);
    color = mix(vec3(0), color,
                smoothstep(edge_thickenss - 0.01, edge_thickenss, m_edge_dist));

    gl_FragColor = vec4(color, 1.0);
}
