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

// based on Slices webcam by stanvanoers
// https://www.shadertoy.com/view/MlG3Wz

vec3 image_color(in vec2 coord) {
    return IMG_NORM_PIXEL(input_image, fract(coord)).rgb;
}

float get_spectrum(float i) {
    return log(IMG_NORM_PIXEL(fft_texture, vec2(fract(i), 0)).x + 1.0);
}

float noise_hash2(vec2 p) {
    p = 50.0 * fract(p * 0.3183099 + vec2(0.71, 0.113));
    return -1.0 + 2.0 * fract(p.x * p.y * (p.x + p.y));
}

float noise21(in vec2 p) {
    vec2 i = floor(p);
    vec2 f = fract(p);
    vec2 u = f * f * (3.0 - 2.0 * f);

    return mix(mix(noise_hash2(i + vec2(0.0, 0.0)),
                   noise_hash2(i + vec2(1.0, 0.0)), u.x),
               mix(noise_hash2(i + vec2(0.0, 1.0)),
                   noise_hash2(i + vec2(1.0, 1.0)), u.x),
               u.y);
}

void main() {
    vec2 st = isf_FragNormCoord;

    // assign the pixel to a slice
    const float slices = 10.0;
    float slice = floor(st.y * slices);
    float s = slice / slices;

    // get a randomish value for this slice
    float n = noise21(vec2(slice, 0.0));

    // compute shift as combo of sin wave and spectral intensity
    const float intensity = 0.1;
    float shift = sin(n * TIME) * intensity - 0.05;
    shift *= log(1.0 + get_spectrum(s) * exp(s)) * 0.2;
    st.x += shift;

    vec3 color = image_color(st);

    gl_FragColor = vec4(color, 1.0);
}
