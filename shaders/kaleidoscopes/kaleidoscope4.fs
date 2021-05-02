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

// based on p6mm kaleidoscope by truename
// https://www.shadertoy.com/view/XdVcRW
#define SQ3 1.7320508076

mat2 rot2d(float a) { return mat2(cos(a), -sin(a), sin(a), cos(a)); }

vec2 p6mm(in vec2 uv, float repeats) {
    uv.x /= SQ3;
    uv = fract(uv * repeats - 0.5) - 0.5;
    uv.x *= SQ3;

    uv = abs(uv);

    vec2 st = uv;

    vec2 uv330 = rot2d(radians(330.0)) * uv;
    if (uv330.x < 0.0) {
        st.y = (st.y - 0.5) * -1.0;
        st.x *= SQ3;
        return st * 2.0;
    }

    return uv;
}

void main() {
    vec3 color = vec3(0.0);

    vec2 st = p6mm(isf_FragNormCoord * 2.0 - 1.0, 2.0);
    color.rg = st;

    gl_FragColor = vec4(color, 1);
}
