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

// based on [SH17A] Fractal Thingy #2 by Klems
// https://www.shadertoy.com/view/Xd2Bzw
void main() {
    vec2 st = isf_FragNormCoord * 2.0 - 1.0;
    st.y *= RENDERSIZE.y / RENDERSIZE.x;
    st *= 0.5;

    vec3 color = vec3(0.0);

    // breathing effect
    st += st * sin(dot(st, st) * 20.0 - TIME) * 0.04;

    const float iterations = 8.0;
    for (float i = 0.5; i < iterations; i++) {
        // fractal formula
        st = abs(2.0 * fract(st - 0.5) - 1.0);

        // rotation
        st *= mat2(cos(TIME * 0.01 * i * i + 0.78 * vec4(1, 7, 3, 1)));

        float spec_strength =
            log(IMG_NORM_PIXEL(fft_texture, vec2(i / iterations, 0.0)).x + 1.0);
        float strength = clamp(spec_strength, 0.0, 1.0) * i;
        color += exp(-abs(st.y) * 5.0) *
                 (cos(vec3(1.0, 3.0, 2.0) * i + TIME * 0.1) * 0.5 + 0.5) *
                 strength;
    }

    color *= 0.5;
    // color.rg *= 0.5;

    gl_FragColor = vec4(color, 1);
}
