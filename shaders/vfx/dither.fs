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

// IQ's palette generator:
// https://www.iquilezles.org/www/articles/palettes/palettes.htm
vec3 palette(in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d) {
    return a + b * cos(6.28318 * (c * t + d));
}

vec3 webcam_color(in vec2 coord) {
    return IMG_NORM_PIXEL(input_image, fract(coord)).rgb;
}

// based on ngMir7 by netgrind
// https://www.shadertoy.com/view/MtXXRf
void main() {
    vec2 st = isf_FragNormCoord;
    st += sin(TIME * vec2(1.0, 1.7)) * 0.01;

    vec3 color = webcam_color(st);

    const float steps = 4.0;

    // take max component and scale it to the step number
    float g = max(color.r, max(color.r, color.b)) * steps;

    // pattern
    float lines = 50.0;
    float f = mod((st.x + st.y + TIME * 0.2) * lines, 1.0);

    if (mod(g, 1.0) > f) {
        color.r = ceil(g);
    } else {
        color.r = floor(g);
    }

    color.r /= steps;

    color = palette(
        color.r, vec3(0.5, 0.5, 0.5), vec3(0.5, 0.5, 0.5), vec3(0.6, 0.8, 1.5),
        fract(vec3(log(IMG_NORM_PIXEL(fft_texture, vec2(0.7, 0)).x + 1.0) + 0.8,
                   log(IMG_NORM_PIXEL(fft_texture, vec2(0.4, 0)).x + 1.0) + 0.9,
                   log(IMG_NORM_PIXEL(fft_texture, vec2(0.1, 0)).x + 1.0) +
                       0.7)));

    gl_FragColor = vec4(color, 1.0);
}
