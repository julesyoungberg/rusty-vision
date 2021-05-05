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

// based on RippleCam by sleep
// https://www.shadertoy.com/view/4djGzz

const vec3 LIGHT_POS = vec3(0.5, 0.5, -1.0);

vec3 image_color(in vec2 coord) {
    return IMG_NORM_PIXEL(inputImage, fract(coord)).rgb;
}

float get_spectrum(float i) {
    return log(IMG_NORM_PIXEL(fft_texture, vec2(fract(i), 0)).x + 1.0);
}

// This height map combines a couple of waves
float height(vec2 st) {
    float s1 = get_spectrum(0.3) * 2.0 + 0.1;
    float s2 = get_spectrum(0.6) * 2.0 + 0.1;
    float t = TIME;

    float shift = sin(st.y * 10.0 + t + cos(st.y * st.x + t)) * s1;
    shift *= sin(st.x * 15.0 + t * 1.387) * s2;
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
    n *= 2.0;
    color.r += image_color(vec2(st.x + n.x, st.y)).r;
    color.g += image_color(vec2(st.x, st.y + n.y)).g;
    color.b += image_color(st - n).b;
    color /= 3.0;

    gl_FragColor = vec4(color, 1.0);
}
