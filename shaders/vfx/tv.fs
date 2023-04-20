/*{
    "DESCRIPTION": "Audio reaactive glitch effects",
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
        },
        {
            "NAME": "shift_speed",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 1.0,
            "DEFAULT": 0.25
        },
        {
            "NAME": "x_shift_speed",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 2.0,
            "DEFAULT": 1.0
        },
        {
            "NAME": "y_shift_speed",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 2.0,
            "DEFAULT": 1.0
        },
        {
            "NAME": "x_flicker_sensitivity",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 2.0,
            "DEFAULT": 1.0
        },
        {
            "NAME": "dispersion_sensitivity",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 2.0,
            "DEFAULT": 1.0
        },
        {
            "NAME": "distortion_sensitivity",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 2.0,
            "DEFAULT": 1.0
        },
        {
            "NAME": "noise_sensitivity",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 2.0,
            "DEFAULT": 1.0
        },
        {
            "NAME": "small_strips_speed",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 2.0,
            "DEFAULT": 1.0
        },
        {
            "NAME": "big_strip_speed",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 2.0,
            "DEFAULT": 1.0
        }
    ]
}*/

// based on Webcam CRT by porglezomp
// https://www.shadertoy.com/view/MdlGRB
// and VCR Distortion by ryk
// https://www.shadertoy.com/view/ldjGzV

vec3 image_color(in vec2 coord) {
    vec2 c = fract(coord);
    return IMG_NORM_PIXEL(inputImage, vec2(c.x, 1.0 - c.y)).rgb;
}

float get_spectrum(float i) {
    return log(IMG_NORM_PIXEL(fft_texture, vec2(fract(i), 0)).x + 1.0);
}

float rand21(vec2 p) {
    return fract(sin(dot(p.xy, vec2(12.9898, 78.233))) * 43758.5453);
}

float noise_hash2(vec2 p) {
    p = 50.0 * fract(p * 0.3183099 + vec2(0.71, 0.113));
    return -1.0 + 2.0 * fract(p.x * p.y * (p.x + p.y));
}

float noise_hash3(vec3 p) {
    p = fract(p * 0.3183099 + .1);
    p *= 17.0;
    return fract(p.x * p.y * p.z * (p.x + p.y + p.z));
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

float noise31(in vec3 x) {
    vec3 i = floor(x);
    vec3 f = fract(x);
    f = f * f * (3.0 - 2.0 * f);

    return mix(mix(mix(noise_hash3(i + vec3(0, 0, 0)),
                       noise_hash3(i + vec3(1, 0, 0)), f.x),
                   mix(noise_hash3(i + vec3(0, 1, 0)),
                       noise_hash3(i + vec3(1, 1, 0)), f.x),
                   f.y),
               mix(mix(noise_hash3(i + vec3(0, 0, 1)),
                       noise_hash3(i + vec3(1, 0, 1)), f.x),
                   mix(noise_hash3(i + vec3(0, 1, 1)),
                       noise_hash3(i + vec3(1, 1, 1)), f.x),
                   f.y),
               f.z);
}

// IQ's pulse function
// https://www.iquilezles.org/www/articles/functions/functions.htm
float pulse(float c, float w, float x) {
    x = abs(x - c);
    if (x > w)
        return 0.0;
    x /= w;
    return 1.0 - x * x * (3.0 - 2.0 * x);
}

float ramp(float y, float start, float end) {
    float inside = step(start, y) - step(end, y);
    float fact = (y - start) / (end - start) * inside;
    return (1.0 - fact) * inside;
}

void main() {
    vec2 st = isf_FragNormCoord;

    // bend the space
    vec2 disp = st - 0.5;
    disp *= sqrt(length(disp));
    st += disp * 1.5;
    st += 0.5;
    st *= 0.5;

    // return black outside the tv frame
    if (!(st.x > 0.0 && st.x < 1.0 && st.y > 0.0 && st.y < 1.0)) {
        gl_FragColor = vec4(vec3(0.0), 1.0);
        float d = max(abs(st.x - 0.5), abs(st.y - 0.5)) - 0.5;
        gl_FragColor += d * 0.1;
        return;
    }

    // save the bent coordinates from this point
    vec2 tv = st;

    // blend the bent coords with originals by sound
    float d = mix(0.5, 1.5, clamp(get_spectrum(0.1) * 0.5, 0.0, 1.0));
    st = mix(isf_FragNormCoord, st, d);

    // apply shifting
    // the window targets a specific horizontal region
    float window = 1.0 / (1.0 + 20.0 * (st.y - mod(TIME * shift_speed, 1.0)) *
                                    (st.y - mod(TIME * shift_speed, 1.0)));
    // start with high freq compound wave
    float x_shift = sin(st.y * 5.0 + TIME * x_shift_speed) / 50.0 *
                    (1.0 + cos(TIME * 80.0 * x_shift_speed));
    x_shift *= window; // concentrate the wave
    x_shift *= step(0.3, get_spectrum(0.3) * x_flicker_sensitivity); // flicker
    st.x += x_shift; // apply shift
    // start with jiggle compound wave
    float y_shift =
        0.4 * sin(TIME * y_shift_speed) * sin(TIME * 20.0 * y_shift_speed);
    y_shift += 0.1 * sin(TIME * 200.0 * cos(TIME)); // add fast flickerywave
    y_shift *= step(0.1, get_spectrum(0.6));        // flicker
    st.y += y_shift;

    // calculate each channel coord to get chromatic shift effect
    float dispersion =
        mix(0.001, 0.1, get_spectrum(0.3) * dispersion_sensitivity);
    vec2 str = st * (1.0 - dispersion) + vec2(dispersion * 0.5);
    vec2 stg = st;
    vec2 stb = st * (1.0 + dispersion) - vec2(dispersion * 0.5);

    // calculate noise effect
    float offset = noise21(vec2(0, st.y + TIME * 155.0));
    float distortion =
        mix(0.0, 0.01, get_spectrum(0.7) * distortion_sensitivity);
    float noisestrength =
        mix(0.0, 0.01, get_spectrum(0.5) * noise_sensitivity);

    // get colors for each channel with noise
    float r =
        mix(image_color(str + offset * distortion).r, offset, noisestrength);
    float g =
        mix(image_color(stg + offset * distortion).g, offset, noisestrength);
    float b =
        mix(image_color(stb + offset * distortion).b, offset, noisestrength);
    vec3 color = vec3(r, g, b);

    // TV noise
    // color += rand21(st) * get_spectrum(0.8);

    // add small TV noise stripes
    float stripes = sin(st.y * 300.0 + TIME * 20.0 * small_strips_speed +
                        sin(TIME * 0.27 * small_strips_speed) * 300.0);
    color = mix(color, vec3(0.8), stripes / 20.0);

    // add big TV noise stripes
    float n = rand21(st) * 0.5 + 0.5;
    float t = st.y + TIME * big_strip_speed * 0.5 +
              sin(TIME * big_strip_speed +
                  sin(TIME * 0.63 * big_strip_speed) * get_spectrum(0.4));
    color = mix(color, vec3(n), pulse(0.5, 0.05, fract(t)) * 0.4);

    // edge fade
    tv *= 2.0;
    tv -= 1.0;
    float v = (1.0 - exp((abs(tv.x) - 1.0) * 3.0)) *
              (1.0 - exp((abs(tv.y) - 1.0) * 3.0));
    color *= mix(0.0, 1.0, v);

    gl_FragColor = vec4(color, 1.0);
}
