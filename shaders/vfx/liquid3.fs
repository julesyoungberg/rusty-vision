/*{
    "DESCRIPTION": "Liquid domain distortion effect.",
    "CREDIT": "by julesyoungberg",
    "ISFVSN": "2.0",
    "CATEGORIES": [ "Distortion" ],
    "INPUTS": [
        {
            "NAME": "inputImage",
            "TYPE": "image"
        },
        {
            "NAME": "speed1",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 2.0,
            "DEFAULT": 1.9
        },
        {
            "NAME": "speed2",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 2.0,
            "DEFAULT": 1.7
        },
        {
            "NAME": "factor_x",
            "TYPE": "float",
            "MIN": 0.01,
            "MAX": 0.1,
            "DEFAULT": 0.07
        },
        {
            "NAME": "factor_y",
            "TYPE": "float",
            "MIN": 0.01,
            "MAX": 0.1,
            "DEFAULT": 0.05
        },
        {
            "NAME": "grid_scale",
            "TYPE": "float",
            "MIN": 0.1,
            "MAX": 10.0,
            "DEFAULT": 6.0
        }
    ]
}*/

#define PI 3.14159265359

vec3 image_color(in vec2 coord) {
    return IMG_NORM_PIXEL(inputImage, fract(coord)).rgb;
}

void main() {
    vec2 st = isf_FragNormCoord;

    float t1 = TIME * speed1;
    float t2 = TIME * speed2;

    for (float i = 1.0; i < 3.0; i += 1.0) {
        vec2 p = st;
        p.x += factor_x / i *
               sin(i * PI * st.y * grid_scale + t1 + sin(t1 * 1.11)) * cos(t1);
        p.y += factor_y / i *
               cos(i * PI * st.x * grid_scale + t2 + sin(t2 * 0.77)) *
               cos(t2 + 1.5);
        st = p;
    }

    vec3 color = image_color(st);

    gl_FragColor = vec4(color, 1.0);
}