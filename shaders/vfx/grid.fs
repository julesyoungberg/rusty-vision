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

vec3 hsv2rgb(vec3 c) {
    vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
    return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}

vec3 rgb2hsv(in vec3 c) {
    vec4 K = vec4(0.0, -1.0 / 3.0, 2.0 / 3.0, -1.0);
    vec4 p = c.g < c.b ? vec4(c.bg, K.wz) : vec4(c.gb, K.xy);
    vec4 q = c.r < p.x ? vec4(p.xyw, c.r) : vec4(c.r, p.yzx);
    float d = q.x - min(q.w, q.y);
    float e = 1.0e-10;
    return vec3(abs(q.z + (q.w - q.y) / (6.0 * d + e)), d / (q.x + e), q.x);
}

vec2 rand2(vec2 p) {
    return fract(
        sin(vec2(dot(p, vec2(127.1, 311.7)), dot(p, vec2(269.5, 183.3)))) *
        43758.5453);
}

float rand21(vec2 p) {
    return fract(sin(dot(p.xy, vec2(12.9898, 78.233))) * 43758.5453);
}

vec3 image_color(in vec2 coord) {
    return IMG_NORM_PIXEL(input_image, fract(coord)).rgb;
}

float get_spectrum(float i) {
    return log(IMG_NORM_PIXEL(fft_texture, vec2(fract(i), 0)).x + 1.0);
}

void main() {
    vec2 st = isf_FragNormCoord * 2.0 - 1.0;
    st.x *= RENDERSIZE.x / RENDERSIZE.y;

    const float scale = 4.0;

    vec3 color = vec3(0.0);

    st *= scale;
    st -= 0.5;
    vec2 gv = fract(st) - 0.5;
    vec2 id = floor(st);

    vec2 coord = isf_FragNormCoord;
    coord += gv * 0.1 * (sin(length(id) * 0.8 - TIME) * 0.5 + 0.5);
    color = image_color(coord);

    vec3 hsv = rgb2hsv(color);
    float i = rand21(id) * 7693.78;
    color = mix(color,
                hsv2rgb(vec3(fract(i + TIME * 0.1 * fract(i)), 1.0, 1.0)), 0.3);
    color *= color;
    color *= get_spectrum(i) * 3.0 + 0.5;

    gl_FragColor = vec4(color, 1.0);
}
