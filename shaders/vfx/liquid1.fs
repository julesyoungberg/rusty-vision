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
        }
    ]
}*/

vec3 image_color(in vec2 coord) {
    vec2 c = fract(coord);
    return IMG_NORM_PIXEL(inputImage, vec2(c.x, 1.0 - c.y)).rgb;
}

void main() {
    vec2 st = isf_FragNormCoord;
    float t = TIME * speed;

    float shift = sin(st.y * factor_y + t + cos(st.y * st.x + t)) * scale1;
    shift *= sin(st.x * factor_x + t * speed_x) * scale2;

    st += shift;

    vec3 color = image_color(st);

    gl_FragColor = vec4(color, 1.0);
}
