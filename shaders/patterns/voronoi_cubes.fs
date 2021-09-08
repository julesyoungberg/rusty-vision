/*{
    "DESCRIPTION": "",
    "CREDIT": "by julesyoungberg",
    "ISFVSN": "2.0",
    "CATEGORIES": [ "GENERATOR" ],
    "INPUTS": [
        {
            "NAME": "scale",
            "TYPE": "float",
            "MIN": 1.0,
            "MAX": 20.0,
            "DEFAULT": 12.0
        },
        {
            "NAME": "mode",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 1.0,
            "DEFAULT": 0.0
        }
    ]
}*/

// based on VoronoiCubes  by mojovideotech
// https://editor.isf.video/shaders/5e7a7ff97c113618206de819

#define C30 0.866025 // cos 30
#define TAU 6.28318530718

vec3 hsv2rgb(vec3 c) {
    vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
    return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}

float rand(float n) { return fract(n * 1183.5437 + .42); }

vec2 rand2(vec2 p) {
    return fract(
        sin(vec2(dot(p, vec2(127.1, 311.7)), dot(p, vec2(269.5, 183.3)))) *
        43758.5453);
}

vec2 get_point(vec2 coord) {
    vec2 point = rand2(coord);
    point = sin(TIME * 0.5 + 6.2831 * point) * 0.5 + 0.5;
    return point;
}

vec4 voronoi(in vec2 p, float mode) {
    vec2 gv = fract(p);
    vec2 id = floor(p);

    float m_corner_dist = 8.0;
    float m_corner_dist2 = 0.0;
    float m_id = 0.0;
    float m_side = 0.0;

    for (float y = -2.0; y <= 2.0; y++) {
        for (float x = -2.0; x <= 2.0; x++) {
            vec2 offset = vec2(x, y);
            vec2 coord = id + offset;
            vec2 point = get_point(coord);
            vec2 diff = offset + point - gv;

            // regular voronoi distance calc
            vec2 d1 = vec2(length(diff), 1.0);
            // voronoi cube distance calc
            vec2 d2 = vec2(max(abs(diff.x) * C30 + diff.y * 0.5, -diff.y),
                           step(0.0, 0.5 * abs(diff.x) + C30 * diff.y) *
                               (1.0 + step(0.0, diff.x)));
            // blend the two modes
            vec2 d = mix(d2, d1, fract(mode));

            // update minimums
            if (d.x < m_corner_dist) {
                m_corner_dist2 = m_corner_dist;
                m_corner_dist = d.x;
                m_id = fract(length(coord));
                m_side = d.y;
            } else if (d.x < m_corner_dist2) {
                m_corner_dist2 = d.x;
            }
        }
    }

    return vec4(m_corner_dist, m_id, m_side * 0.5,
                m_corner_dist2 - m_corner_dist);
}

void main() {
    vec2 st = isf_FragNormCoord * 2.0 - 1.0;
    st.y *= RENDERSIZE.y / RENDERSIZE.x;
    st = st * 0.5 + 0.5;

    vec4 val = voronoi(st * scale, mode);

    // unique cell color
    vec3 color = sin(val.y * 10.0 + vec3(2, 1, 0.5) + TIME) * 0.5 + 0.5;
    // slide edge shading
    color *= sqrt(clamp(1.0 - val.x, 0.0, 1.0));
    // cube face shading
    color *= clamp((1.0 - val.z) * 0.5 + 0.5, 0.0, 1.0);

    gl_FragColor = vec4(color, 1.0);
}
