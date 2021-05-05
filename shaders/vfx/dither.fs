/*{
    "DESCRIPTION": "Colorful dither effect.",
    "CREDIT": "by julesyoungberg",
    "ISFVSN": "2.0",
    "CATEGORIES": [ "Halftone Effect" ],
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
            "NAME": "color_offset",
            "TYPE": "color",
            "DEFAULT": [0.5, 0.9, 0.0, 0.0]
        },
        {
            "NAME": "num_lines",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 100.0,
            "DEFAULT": 50.0
        },
        {
            "NAME": "steps",
            "TYPE": "float",
            "MIN": 1.0,
            "MAX": 10.0,
            "DEFAULT": 4.0
        },
        {
            "NAME": "speed",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 1.0,
            "DEFAULT": 0.2
        }
    ]
}*/

// IQ's palette generator:
// https://www.iquilezles.org/www/articles/palettes/palettes.htm
vec3 palette(in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d) {
    return a + b * cos(6.28318 * (c * t + d));
}

vec3 image_color(in vec2 coord) {
    return IMG_NORM_PIXEL(inputImage, fract(coord)).rgb;
}

// based on ngMir7 by netgrind
// https://www.shadertoy.com/view/MtXXRf
void main() {
    vec2 st = isf_FragNormCoord;
    st += sin(TIME * vec2(1.0, 1.7)) * 0.01;

    vec3 color = image_color(st);

    // take max component and scale it to the step number
    float g = max(color.r, max(color.r, color.b)) * steps;

    // pattern
    float f = fract((st.x + st.y + TIME * speed) * num_lines);

    if (mod(g, 1.0) > f) {
        color.r = ceil(g);
    } else {
        color.r = floor(g);
    }

    color.r /= steps;

    color = palette(
        color.r, vec3(0.5, 0.5, 0.5), vec3(0.5, 0.5, 0.5), vec3(0.9, 0.8, 1.0),
        fract(vec3(log(IMG_NORM_PIXEL(fft_texture, vec2(0.7, 0)).x + 1.0) +
                       color_offset.r,
                   log(IMG_NORM_PIXEL(fft_texture, vec2(0.4, 0)).x + 1.0) +
                       color_offset.g,
                   log(IMG_NORM_PIXEL(fft_texture, vec2(0.1, 0)).x + 1.0) +
                       color_offset.b)));

    gl_FragColor = vec4(color, 1.0);
}
