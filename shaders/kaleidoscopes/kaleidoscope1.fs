/*{
    "DESCRIPTION": "",
    "CREDIT": "by julesyoungberg",
    "ISFVSN": "2.0",
    "CATEGORIES": [ "GENERATOR" ],
    "INPUTS": [
        {
            "NAME": "fft_texture",
            "TYPE": "audioFFT"
        },
        {
            "NAME": "scale_factor",
            "TYPE": "float",
            "MIN": 0.1,
            "MAX": 2.0,
            "DEFAULT": 0.5
        },
        {
            "NAME": "zoom_speed",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 1.0,
            "DEFAULT": 0.5
        },
        {
            "NAME": "rotate_speed",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 1.0,
            "DEFAULT": 0.1
        },
        {
            "NAME": "rotate_amount",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 1.0,
            "DEFAULT": 1.0
        },
        {
            "NAME": "sensitivity",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 1.0,
            "DEFAULT": 0.5
        },
        {
            "NAME": "rOffset",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 2.0,
            "DEFAULT": 0.7
        },
        {
            "NAME": "gOffset",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 2.0,
            "DEFAULT": 0.4
        },
        {
            "NAME": "bOffset",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 2.0,
            "DEFAULT": 0.1
        }
    ]
}*/

#define PI 3.14159265359
#define ITERATIONS 10

// based on Kaleidoscope Illusion by tiff
// https://www.shadertoy.com/view/llGcRK
void main() {
    vec2 st = isf_FragNormCoord * 2.0 - 1.0;
    st.y *= RENDERSIZE.y / RENDERSIZE.x;
    st *= scale_factor;
    st *= cos(TIME * zoom_speed) + 1.5;

    vec3 color = vec3(0.0);

    float scale = PI / 3;
    float m = 0.5;

    for (float i = 0.0; i < ITERATIONS; i += 1.0) {
        float angle = mod(i + TIME * rotate_speed + 1.5, PI * 2.0);
        float c = cos(angle);
        float s = sin(angle);
        st = mix(st, st * mat2(c, -s, s, c), rotate_amount);

        float theta = atan(st.x, st.y) + PI;
        // round theta off to scale pieces
        theta = (floor(theta / scale) + 0.5) * scale;

        vec2 dir = vec2(sin(theta), cos(theta));
        vec2 codir = dir.yx * vec2(-1, 1);

        st = vec2(dot(dir, st), dot(codir, st));
        st += vec2(sin(TIME * 0.1), cos(TIME * 0.15)) * 0.035 * sin(angle);
        st = abs(fract(st + 0.5) * 2.0 - 1.0) * 0.7;
        // st = fract(st);

        float spec1 = log(
            IMG_NORM_PIXEL(fft_texture, vec2(((i * 3)) / (ITERATIONS * 3), 0))
                    .x *
                sensitivity +
            rOffset);
        float spec2 =
            log(IMG_NORM_PIXEL(fft_texture,
                               vec2(((i * 3) + 1.0) / (ITERATIONS * 3), 0))
                        .x *
                    sensitivity +
                gOffset);
        float spec3 =
            log(IMG_NORM_PIXEL(fft_texture,
                               vec2(((i * 3) + 2.0) / (ITERATIONS * 3), 0))
                        .x *
                    sensitivity +
                bOffset);
        vec3 p = vec3(spec1, spec2, spec3) * vec3(1.1, 1.7, 2.3) * 0.5;
        color += exp(-min(st.x, st.y) * 16.0) *
                 (cos(p + i + TIME * 0.5) * 0.5 + 0.5) * m;
        m *= 0.7;
    }

    gl_FragColor = vec4(color * color, 1.0);
}
