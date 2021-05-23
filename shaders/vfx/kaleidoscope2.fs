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
            "NAME": "angle_speed",
            "TYPE": "float",
            "MIN": -0.5,
            "MAX": 0.5,
            "DEFAULT": 0.01
        },
        {
            "NAME": "x_speed",
            "TYPE": "float",
            "MIN": -1.0,
            "MAX": 1.0,
            "DEFAULT": 0.5
        },
        {
            "NAME": "y_speed",
            "TYPE": "float",
            "MIN": -1.0,
            "MAX": 1.0,
            "DEFAULT": 0.7
        },
        {
            "NAME": "scale1",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 1.0,
            "DEFAULT": 0.035
        },
        {
            "NAME": "scale2",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 1.0,
            "DEFAULT": 0.7
        },
        {
            "NAME": "color_speed",
            "TYPE": "float",
            "MIN": -0.2,
            "MAX": 0.2,
            "DEFAULT": 0.05
        }
    ]
}*/

// based on Kaleidoscope Illusion by tiff
// https://www.shadertoy.com/view/llGcRK

#define PI 3.14159265359

vec3 image_color(in vec2 coord) {
    return IMG_NORM_PIXEL(inputImage, fract(coord)).rgb;
}

void main() {
    vec2 st = isf_FragNormCoord * 2.0 - 1.0;
    st.x *= RENDERSIZE.x / RENDERSIZE.y;
    // st *= cos(TIME * 0.1) + 1.5;

    vec3 color = vec3(0.0);

    float scale = PI / 3.0;

    for (float i = 0.0; i < 3.0; i += 1.0) {
        float scaleFactor = i; // + sin(TIME * 0.05) + 1.5;

        float angle = TIME * scaleFactor * angle_speed;
        st *= mat2(cos(angle + PI * 0.25 * vec4(0, 6, 2, 0)));

        float theta = atan(st.x, st.y) + PI;
        theta = (floor(theta / scale) + 0.5) * scale;

        vec2 dir = vec2(sin(theta), cos(theta));
        vec2 codir = dir.yx * vec2(-1, 1);

        st = vec2(dot(dir, st), dot(codir, st));
        st.xy += vec2(sin(TIME * x_speed), cos(TIME * y_speed)) * scaleFactor * scale1;
        st = abs(fract(st + 0.5) * 2.0 - 1.0) * scale2;
    }

    color = image_color(mod(st * 0.5 - TIME * color_speed, 1.0));

    gl_FragColor = vec4(color, 1.0);
}
