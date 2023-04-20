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
            "NAME": "speed",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 2.0,
            "DEFAULT": 1.0
        },
        {
            "NAME": "speed_x",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 2.0,
            "DEFAULT": 1.387
        },
        {
            "NAME": "scale1",
            "TYPE": "float",
            "MIN": 0.01,
            "MAX": 1.0,
            "DEFAULT": 0.1
        },
        {
            "NAME": "scale2",
            "TYPE": "float",
            "MIN": 0.01,
            "MAX": 1.0,
            "DEFAULT": 0.1
        },
        {
            "NAME": "factor_y",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 20.0,
            "DEFAULT": 10.0
        },
        {
            "NAME": "factor_x",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 20.0,
            "DEFAULT": 15.0
        },
        {
            "NAME": "normal_scale",
            "TYPE": "float",
            "MIN": 1.0,
            "MAX": 5.0,
            "DEFAULT": 2.0
        }
    ]
}*/

// based on RippleCam by sleep
// https://www.shadertoy.com/view/4djGzz

const vec3 LIGHT_POS = vec3(0.5, 0.5, -1.0);

vec3 image_color(in vec2 coord) {
    vec2 c = fract(coord);
    return IMG_NORM_PIXEL(inputImage, vec2(c.x, 1.0 - c.y)).rgb;
}

// This height map combines a couple of waves
float height(vec2 st) {
    float t = TIME * speed;
    float shift = sin(st.y * factor_y + t + cos(st.y * st.x + t)) * scale1;
    shift *= sin(st.x * factor_x + t * speed_x) * scale2;
    return shift;
}

// Discrete differentiation
vec2 normal(vec2 pos) {
    return vec2(height(pos - vec2(0.01, 0)) - height(pos),
                height(pos - vec2(0, 0.01)) - height(pos));
}

void main() {
    vec2 st = isf_FragNormCoord;

    vec3 color = vec3(0.0);

    vec2 n = normal(isf_FragNormCoord * 2.0 - 1.0);
    color = image_color(st + n);
    n *= normal_scale;
    color.r += image_color(vec2(st.x + n.x, st.y)).r;
    color.g += image_color(vec2(st.x, st.y + n.y)).g;
    color.b += image_color(st - n).b;
    color /= 3.0;

    gl_FragColor = vec4(color, 1.0);
}
