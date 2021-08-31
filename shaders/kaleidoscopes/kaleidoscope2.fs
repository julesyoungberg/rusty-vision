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
            "NAME": "iterations",
            "TYPE": "float",
            "MIN": 1.0,
            "MAX": 10.0,
            "DEFAULT": 8.0
        },
        {
            "NAME": "scale",
            "TYPE": "float",
            "MIN": 0.1,
            "MAX": 5.0,
            "DEFAULT": 0.5
        },
        {
            "NAME": "breath_amount",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 100.0,
            "DEFAULT": 20.0
        },
        {
            "NAME": "breath_speed",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 2.0,
            "DEFAULT": 1.0
        },
        {
            "NAME": "speed",
            "TYPE": "float",
            "MIN": -0.1,
            "MAX": 0.1,
            "DEFAULT": 0.01
        },
        {
            "NAME": "color_speed",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 1.0,
            "DEFAULT": 0.1
        },
        {
            "NAME": "sensitivity",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 1.0,
            "DEFAULT": 0.5
        },
        {
            "NAME": "color_scale",
            "TYPE": "color",
            "DEFAULT": [
                0.1,
                0.3,
                0.2,
                0.1
            ]
        },
        {
            "NAME": "angles",
            "TYPE": "color",
            "DEFAULT": [
                0.1,
                0.7,
                0.3,
                0.1
            ]
        }
    ]
}*/

// based on [SH17A] Fractal Thingy #2 by Klems
// https://www.shadertoy.com/view/Xd2Bzw
void main() {
    vec2 st = isf_FragNormCoord * 2.0 - 1.0;
    st.y *= RENDERSIZE.y / RENDERSIZE.x;
    st *= scale;

    vec3 color = vec3(0.0);

    float t = TIME * breath_speed;

    // breathing effect
    st += st * sin(dot(st, st) * breath_amount - t) * 0.04;

    for (float i = 0.5; i < iterations; i++) {
        // fractal formula
        st = abs(2.0 * fract(st - 0.5) - 1.0);

        // rotation
        st *= mat2(cos(TIME * speed * i + 0.78 * angles * 10.0));

        float spec_strength =
            log(IMG_NORM_PIXEL(fft_texture, vec2(i / iterations, 0.0)).x + 1.0);
        float strength =
            mix(1.0 - sensitivity, 1.0, clamp(spec_strength, 0.0, 1.0) * i);
        color +=
            exp(-abs(st.y) * 5.0) *
            (cos(color_scale.rgb * 10.0 * i + TIME * color_speed) * 0.5 + 0.5) *
            strength;
    }

    color *= 0.5;
    // color.rg *= 0.5;

    gl_FragColor = vec4(color, 1);
}
