/*{
    "DESCRIPTION": "Image kifs effect.",
    "CREDIT": "by julesyoungberg",
    "ISFVSN": "2.0",
    "CATEGORIES": [ "Kaleidoscope" ],
    "INPUTS": [
        {
            "NAME": "inputImage",
            "TYPE": "image"
        },
        {
            "NAME": "grid_scale",
            "TYPE": "float",
            "MIN": 1.0,
            "MAX": 10.0,
            "DEFAULT": 2.0
        },
        {
            "NAME": "factor",
            "TYPE": "float",
            "MIN": 0.1,
            "MAX": 10.0,
            "DEFAULT": 3.0
        },
        {
            "NAME": "shift",
            "TYPE": "float",
            "MIN": -5.0,
            "MAX": 5.0,
            "DEFAULT": 1.5
        },
        {
            "NAME": "speed",
            "TYPE": "float",
            "MIN": -1.0,
            "MAX": 1.0,
            "DEFAULT": -0.05
        }
    ]
}*/

#define PI 3.14159265359

vec2 N(float angle) { return vec2(sin(angle), cos(angle)); }

float sdBox(in vec2 p, in vec2 b) {
    vec2 d = abs(p) - b;
    return length(max(d, 0.0)) + min(max(d.x, d.y), 0.0);
}

vec3 image_color(in vec2 coord) {
    return IMG_NORM_PIXEL(inputImage, fract(coord)).rgb;
}

void main() {
    vec2 st = isf_FragNormCoord * 2.0 - 1.0;
    st.y *= RENDERSIZE.y / RENDERSIZE.x;
    st *= grid_scale;
    vec3 color = vec3(0);
    vec2 size = vec2(0.5);

    float scale = 1.0;
    float dist = 100.0;

    for (int i = 0; i < 10; i++) {
        dist = min(dist, sdBox(st, size) * scale);

        float angle = TIME * 0.1;
        float c = cos(angle);
        float s = sin(angle);
        st *= mat2(c, -s, s, c);

        st.x = abs(st.x);
        st.y = abs(st.y);

        st *= factor;
        scale /= factor;
        st -= shift;
    }

    st *= scale;
    color = image_color(st + TIME * speed);
    // color += 1.0 - sign(dist);

    gl_FragColor = vec4(color, 1.0);
}
