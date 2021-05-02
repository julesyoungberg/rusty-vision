/*{
    "DESCRIPTION": "",
    "CREDIT": "by julesyoungberg",
    "ISFVSN": "2.0",
    "CATEGORIES": [ "GENERATOR" ],
    "INPUTS": [
        {
            "NAME": "fft_texture",
            "TYPE": "audioFFT",
            "MAX": 32
        }
    ]
}*/

// based on Illustrated Equations by sben
// https://www.shadertoy.com/view/MtBGDW0

#define SPECTRUM_SIZE 32

// IQ's palette generator:
// https://www.iquilezles.org/www/articles/palettes/palettes.htm
vec3 palette(in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d) {
    return a + b * cos(6.28318 * (c * t + d));
}

void main() {
    vec2 st = isf_FragNormCoord * 2.0 - 1.0;
    st.x *= RENDERSIZE.x / RENDERSIZE.y;

    vec3 color = vec3(0.0);

    float strength = log(IMG_NORM_PIXEL(fft_texture, vec2(0.04, 0)).x + 1.0);

    st *= 10.0;
    st *= mix(1.0, 0.95, strength);

    vec2 p = abs(st * 2.0);
    vec2 ab = vec2(2.0 - p.x);
    float t = TIME;

    for (float i = 0.0; i < 12.0; i++) {
        ab += p + cos(length(p) - t) * sin(t * 0.1);
        p.y += sin(ab.x - p.x - t) * 0.5;
        p.x += sin(ab.y - t) * 0.5;
        p -= p.x + p.y;
        p += (st.y + cos(st.x) * sin(t * 0.267)) * sin(t * 0.345);
        ab += vec2(p.y);
    }

    p /= 30.0;

    float id = p.x * 2.0 + p.y;

    color = palette(
        id, vec3(0.5, 0.5, 0.5), vec3(0.5, 0.5, 0.5), vec3(1.0, 1.0, 1.0),
        fract(vec3(log(IMG_NORM_PIXEL(fft_texture, vec2(0.7, 0)).x + 1.0),
                   log(IMG_NORM_PIXEL(fft_texture, vec2(0.4, 0)).x + 1.0),
                   log(IMG_NORM_PIXEL(fft_texture, vec2(0.1, 0)).x + 1.0))));

    float size = mix(0.1, 0.8, strength);
    color = mix(color, vec3(0.0), smoothstep(size, size + 0.01, id));
    color *= vec3(0.5, 1.0, 1.2);

    gl_FragColor = vec4(sqrt(color), 1);
}
