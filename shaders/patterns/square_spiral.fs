/*{
    "DESCRIPTION": "",
    "CREDIT": "by julesyoungberg",
    "ISFVSN": "2.0",
    "CATEGORIES": [ "GENERATOR" ],
    "INPUTS": [
        {
            "NAME": "fft_texture",
            "TYPE": "audioFFT"
        }
    ]
}*/

#define PI 3.14159265359
#define ITERATIONS 32.0

vec3 hsv2rgb(vec3 c) {
    vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
    return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}

float sdBox(in vec2 p, in vec2 b) {
    vec2 d = abs(p) - b;
    return length(max(d, 0.0)) + min(max(d.x, d.y), 0.0);
}

float square(in vec2 p, in float width) {
    float dist = sdBox(p, vec2(1));
    return smoothstep(width, 0.0, dist) - smoothstep(0.0, -width, dist);
}

float get_spectrum(float i) {
    return log(IMG_NORM_PIXEL(fft_texture, vec2(fract(i), 0)).x + 1.0);
}

void main() {
    vec2 st = isf_FragNormCoord * 2.0 - 1.0;
    st.y *= RENDERSIZE.y / RENDERSIZE.x;
    st *= 10.0;

    float angle = TIME;
    float c = cos(angle);
    float s = sin(angle);
    st *= mat2(c, -s, s, c);

    vec3 color = vec3(0);

    for (float i = 0.0; i < ITERATIONS; i += 1.0) {
        float m = mod(i * 3.2, ITERATIONS);
        float intensity = get_spectrum(m / ITERATIONS);
        color += square(st, 0.006 * intensity) *
                 hsv2rgb(vec3(mod(i / ITERATIONS - TIME * 0.3, 1.0), 1, 1)) *
                 sqrt(intensity * 0.5) * 0.1 * (m + 1.0);

        angle = (i + 1) * PI * 0.002 * sin(TIME * 0.5);
        c = cos(angle);
        s = sin(angle);
        st *= mat2(c, -s, s, c);
        st *= (sin(TIME * 0.7) * 0.5 + 0.5) * 0.04 + 0.92;
    }

    gl_FragColor = vec4(color, 1.0);
}