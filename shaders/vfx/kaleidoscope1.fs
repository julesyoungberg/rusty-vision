/*{
    "DESCRIPTION": "Image kaleidoscope effect.",
    "CREDIT": "by julesyoungberg",
    "ISFVSN": "2.0",
    "CATEGORIES": [ "Kaleidoscope" ],
    "INPUTS": [
        {
            "NAME": "inputImage",
            "TYPE": "image"
        }
    ]
}*/

#define PI 3.14159265359

// https://www.shadertoy.com/view/MtKXDR
vec2 kaleidoscope(vec2 st) {
    float a = atan(st.y, st.x);
    float r = pow(length(st), 0.9);
    float p = sin(2.0 * PI * TIME * 0.02);
    float q = 2.0 * PI / 9.0;
    a = abs(mod(a, q) - 0.5 * q);
    float factor = pow(r, 1.3) * 0.1;
    return vec2(cos(a), sin(a)) * factor;
}

vec2 transform(vec2 st) {
    float a = TIME * 0.02;
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
