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
            "MAX": 50.0,
            "DEFAULT": 20.0
        },
        {
            "NAME": "speed",
            "TYPE": "float",
            "MIN": -2.0,
            "MAX": 2.0,
            "DEFAULT": -1.5
        },
        {
            "NAME": "color_speed",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 2.0,
            "DEFAULT": 0.5
        },
        {
            "NAME": "initial_angle",
            "TYPE": "float",
            "MIN": -3.1415926536,
            "MAX": 3.1415926536,
            "DEFAULT": 0.7853981634
        },
        {
            "NAME": "color_params",
            "TYPE": "color",
            "DEFAULT": [
                0.1,
                0.8,
                0.7,
                0.0
            ]
        }
    ]
}*/

#define PI 3.14159265359

float Xor(float a, float b) { return a * (1.0 - b) + b * (1.0 - a); }

void main() {
    vec2 st = isf_FragNormCoord * 2.0 - 1.0;
    st.y *= RENDERSIZE.y / RENDERSIZE.x;

    float angle = initial_angle;
    float c = cos(angle);
    float s = sin(angle);
    st *= mat2(c, -s, s, c);
    st *= scale;

    vec3 color = vec3(0);

    vec2 gv = fract(st) - 0.5;
    vec2 id = floor(st);
    float m = 0;
    float t = TIME * speed;

    for (float y = -1.0; y <= 1.0; y++) {
        for (float x = -1.0; x <= 1.0; x++) {
            vec2 offset = vec2(x, y);
            vec2 lid = id + offset;
            vec2 lgv = gv - offset;

            angle = TIME;
            c = cos(angle);
            s = sin(angle);
            lgv *= mat2(c, -s, s, c);

            float center_dist = length(lid) * 0.5;
            float r = mix(0.5, 1.5, sin(t + center_dist) * 0.5 + 0.5);
            float circle_dist = length(lgv);
            m = Xor(smoothstep(r, r * 0.95, circle_dist), m);
        }
    }

    // color += m;
    color = sin(TIME * color_speed + m * PI * color_params.rgb + length(gv) +
                PI * 0.5) *
                0.5 +
            0.5;

    gl_FragColor = vec4(color, 1);
}
