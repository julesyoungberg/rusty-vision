/*{
    "DESCRIPTION": "Audio reaactive glitch effects",
    "CREDIT": "by julesyoungberg",
    "ISFVSN": "2.0",
    "CATEGORIES": [ "Glitch" ],
    "INPUTS": [
        {
            "NAME": "inputImage",
            "TYPE": "image"
        },
        {
            "NAME": "fft_texture",
            "TYPE": "audioFFT"
        },
        {
            "NAME": "speed",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 2.0,
            "DEFAULT": 1.0
        },
        {
            "NAME": "freq",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 1.0,
            "DEFAULT": 0.2
        },
        {
            "NAME": "sensitivity",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 3.0,
            "DEFAULT": 1.0
        },
        {
            "NAME": "pieces",
            "TYPE": "float",
            "MIN": 1.0,
            "MAX": 8.0,
            "DEFAULT": 4.0
        }
    ]
}*/

// based on ngMir8 by netgrind
// https://www.shadertoy.com/view/XtlSzX

#define TAU 6.28318530718

vec3 image_color(in vec2 coord) {
    return IMG_NORM_PIXEL(inputImage, fract(coord)).rgb;
}

float get_spectrum(float i) {
    return log(IMG_NORM_PIXEL(fft_texture, vec2(fract(i), 0)).x + 1.0);
}

void main() {
    vec2 st = isf_FragNormCoord;

    vec3 color = image_color(st);

    float t = TIME * speed;
    float d = mix(0.01, 0.1, get_spectrum(freq) * sensitivity);

    for (float i = 0.0; i < TAU; i += TAU / pieces) {
        float a = i + t;
        vec3 color2 = image_color(vec2(st.x + cos(a) * d, st.y + sin(a) * d));
        color = min(color, color2);
    }

    gl_FragColor = vec4(color, 1.0);
}
