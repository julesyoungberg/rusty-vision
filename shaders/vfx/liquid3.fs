/*{
    "DESCRIPTION": "Audio reaactive glitch effects",
    "CREDIT": "by julesyoungberg",
    "ISFVSN": "2.0",
    "CATEGORIES": [ "FX" ],
    "INPUTS": [
        {
            "NAME": "input_image",
            "TYPE": "image"
        }
    ]
}*/

#define PI 3.14159265359

vec3 image_color(in vec2 coord) {
    return IMG_NORM_PIXEL(input_image, fract(coord)).rgb;
}

void main() {
    vec2 st = isf_FragNormCoord;

    float t1 = TIME * 1.9;
    float t2 = TIME * 1.7;

    for (float i = 1.0; i < 3.0; i += 1.0) {
        vec2 p = st;
        p.x +=
            0.07 / i * sin(i * PI * st.y * 6.0 + t1 + sin(t1 * 1.11)) * cos(t1);
        p.y += 0.05 / i * cos(i * PI * st.x * 6.0 + t2 + sin(t2 * 0.77)) *
               cos(t2 + 1.5);
        st = p;
    }

    vec3 color = image_color(st);

    gl_FragColor = vec4(color, 1.0);
}