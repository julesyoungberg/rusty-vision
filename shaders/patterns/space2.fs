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
            "NAME": "initial_angle",
            "TYPE": "float",
            "MIN": -3.1415926536,
            "MAX": 3.1415926536,
            "DEFAULT": 0.7853981634
        },
        {
            "NAME": "shift_speed",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 2.0,
            "DEFAULT": 1.0
        },
        {
            "NAME": "shift_amount",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 0.1,
            "DEFAULT": 0.01
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

    vec3 color = vec3(0);

    for (float i = 0; i < 3; i++) {
        vec2 p = st;

        float shift =
            sin(i * PI + TIME * (i + 0.1) * shift_speed + length(p) * 5.0) *
            shift_amount;
        p += shift;

        p *= scale;

        vec2 gv = fract(p) - 0.5;
        vec2 id = floor(p);
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

                float center_dist = length(lid) * 0.3;
                float r = mix(0.5, 1.5, sin(t + center_dist) * 0.5 + 0.5);
                float circle_dist = length(lgv);
                m = Xor(smoothstep(r, r * 0.95, circle_dist), m);
            }
        }

        color[int(i)] += m;
    }

    gl_FragColor = vec4(color, 1);
}
