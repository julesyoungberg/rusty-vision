/*{
    "DESCRIPTION": "Audio reaactive glitch effects",
    "CREDIT": "by julesyoungberg",
    "ISFVSN": "2.0",
    "CATEGORIES": [ "FX" ],
    "INPUTS": [
        {
            "NAME": "fft_texture",
            "TYPE": "audioFFT"
        },
        {
            "NAME": "input_image",
            "TYPE": "image"
        }
    ]
}*/

// based on ngMir8 by netgrind
// https://www.shadertoy.com/view/XtlSzX

#define TAU 6.28318530718

vec3 image_color(in vec2 coord) {
    return IMG_NORM_PIXEL(input_image, fract(coord)).rgb;
}

float get_spectrum(float i) {
    return log(IMG_NORM_PIXEL(fft_texture, vec2(fract(i), 0)).x + 1.0);
}

void main() {
    vec2 st = isf_FragNormCoord;

    vec3 color = image_color(st);

    float t = TIME;
    float d = mix(0.01, 0.1, get_spectrum(0.1));

    const float taps = 6.0;

    for (float i = 0.0; i < TAU; i += TAU / taps) {
        float a = i + t;
        vec3 color2 = image_color(vec2(st.x + cos(a) * d, st.y + sin(a) * d));
        color = min(color, color2);
    }

    gl_FragColor = vec4(color, 1.0);
}
