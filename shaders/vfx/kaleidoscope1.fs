/*{
    "DESCRIPTION": "Image kaleidoscope effect.",
    "CREDIT": "by julesyoungberg",
    "ISFVSN": "2.0",
    "CATEGORIES": [ "Kaleidoscope" ],
    "INPUTS": [
        {
            "NAME": "inputImage",
            "TYPE": "image"
        },
        {
            "NAME": "speed",
            "TYPE": "float",
            "MIN": -0.2,
            "MAX": 0.2,
            "DEFAULT": 0.05
        },
        {
            "NAME": "slices",
            "TYPE": "float",
            "MIN": 1.0,
            "MAX": 20.0,
            "DEFAULT": 9.0
        },
        {
            "NAME": "power",
            "TYPE": "float",
            "MIN": 0.01,
            "MAX": 5.0,
            "DEFAULT": 1.3
        },
        {
            "NAME": "factor_strength",
            "TYPE": "float",
            "MIN": 0.001,
            "MAX": 1.0,
            "DEFAULT": 0.1
        }
    ]
}*/

#define PI 3.14159265359

// https://www.shadertoy.com/view/MtKXDR
vec2 kaleidoscope(vec2 st) {
    float a = atan(st.y, st.x);
    float r = pow(length(st), 0.9);
    float q = 2.0 * PI / slices;
    a = abs(mod(a, q) - 0.5 * q);
    float factor = pow(r, power) * factor_strength;
    return vec2(cos(a), sin(a)) * factor;
}

vec2 transform(vec2 st) {
    float a = TIME * speed;
    vec2 v;
    v.x = st.x * cos(a) - st.y * sin(a) - 0.3 * sin(a);
    v.y = st.x * sin(a) + st.y * cos(a) + 0.3 * cos(a);
    return v;
}

vec4 image_color(in vec2 coord) {
    return IMG_NORM_PIXEL(inputImage, fract(coord));
}

vec4 scene(vec2 st) { return image_color(transform(st)); }

void main() {
    vec2 st = isf_FragNormCoord * 2.0 - 1.0;
    st.y *= RENDERSIZE.y / RENDERSIZE.x;
    gl_FragColor = scene(kaleidoscope(st));
}
