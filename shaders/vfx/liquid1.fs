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
            "NAME": "fft_texture",
            "TYPE": "audioFFT"
        }
    ]
}*/

vec3 image_color(in vec2 coord) {
    return IMG_NORM_PIXEL(inputImage, fract(coord)).rgb;
}

float get_spectrum(float i) {
    return log(IMG_NORM_PIXEL(fft_texture, vec2(fract(i), 0)).x + 1.0);
}

void main() {
    vec2 st = isf_FragNormCoord;

    float s1 = get_spectrum(0.3) * 2.0 + 0.1;
    float s2 = get_spectrum(0.6) * 2.0 + 0.1;
    float t = TIME;

    float shift = sin(st.y * 10.0 + t + cos(st.y * st.x + t)) * s1;
    shift *= sin(st.x * 15.0 + t * 1.387) * s2;

    st += shift;

    vec3 color = image_color(st);

    gl_FragColor = vec4(color, 1.0);
}
