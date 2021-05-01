/*{
    "DESCRIPTION": "",
    "CREDIT": "by julesyoungberg",
    "ISFVSN": "2.0",
    "CATEGORIES": [ "GENERATOR" ],
    "INPUTS": [
        {
            "NAME": "t1_speed",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 2.0,
            "DEFAULT": 1.13
        },
        {
            "NAME": "t2_speed",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 2.0,
            "DEFAULT": 0.9
        },
        {
            "NAME": "iterations",
            "TYPE": "float",
            "MIN": 1.0,
            "MAX": 10.0,
            "DEFAULT": 5.0
        },
        {
            "NAME": "color_shift1",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 15.0,
            "DEFAULT": 8.69
        },
        {
            "NAME": "color_shift2",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 15.0,
            "DEFAULT": 2.33
        },
        {
            "NAME": "color_shift3",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 15.0,
            "DEFAULT": 13.0
        },
        {
            "NAME": "color_shift4",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 15.0,
            "DEFAULT": 4.69
        }
    ]
}*/

// based on ColorDiffusionFlow by mojovideotech
// https://editor.isf.video/shaders/5e7a80437c113618206dee05

#define PI 3.14159265359

void main() {
    vec2 st = isf_FragNormCoord;
    st.y *= RENDERSIZE.y / RENDERSIZE.x;
    st *= 2.0;

    vec3 color = vec3(0.0);

    float t1 = TIME * t1_speed;
    float t2 = TIME * t2_speed;

    for (float i = 1.0; i < iterations; i += 1.0) {
        vec2 p = st;
        p.x += 0.75 / i * sin(i * PI * st.y + t1 * 0.1);
        p.y += 0.35 / i * cos(i * PI * st.x + t2);
        st = p;
    }

    float v = st.x + st.y;
    color = vec3(cos(v + color_shift1) * 0.5 + 0.5,
                 sin(v + color_shift2) * 0.5 + 0.5,
                 (sin(v + color_shift3) + cos(v + color_shift4)) * 0.25 + 0.5);

    gl_FragColor = vec4(color, 1.0);
}
