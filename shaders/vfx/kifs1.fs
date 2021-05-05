/*{
    "DESCRIPTION": "Image kifs effect.",
    "CREDIT": "by julesyoungberg",
    "ISFVSN": "2.0",
    "CATEGORIES": [ "Kaleidoscope" ],
    "INPUTS": [
        {
            "NAME": "inputImage",
            "TYPE": "image"
        }
    ]
}*/

#define PI 3.14159265359

vec2 N(float angle) { return vec2(sin(angle), cos(angle)); }

vec3 image_color(in vec2 coord) {
    return IMG_NORM_PIXEL(inputImage, fract(coord)).rgb;
}

void main() {
    vec2 st = isf_FragNormCoord * 2.0 - 1.0;
    st.y *= RENDERSIZE.y / RENDERSIZE.x;
    st *= 1.5;
    vec3 color = vec3(0);

    st.x = abs(st.x);
    st.y += tan(5.0 / 6.0 * PI) * 0.5;
    vec2 n = N(5.0 / 6.0 * PI);
    float d = dot(st - vec2(0.5, 0.0), n);
    st -= n * max(d, 0.0) * 2.0;

    n = N(2.0 / 3.0 * PI); // sin(TIME * 0.0) * PI);
    float factor = 3.0;    // (sin(TIME * 0.01) * 0.5 + 0.75) * 5.0;
    float shift = -1.5;    // * sin(TIME * 0.05 - 1.5);
    float scale = 1.0;
    st.x -= shift / factor; // compensate for -= 1.5
    for (int i = 0; i < 4; i++) {
        st *= factor;
        scale *= factor;
        st.x += shift;

        st.x = abs(st.x);
        st.x -= 0.5;
        st -= n * min(dot(st, n), 0.0) * 2.0;
    }

    st /= scale;
    // d = length(st - vec2(clamp(st.x, -1.0, 1.0), 0));
    // color += smoothstep(1.0 / resolution.y, 0.0, d / scale);
    color = image_color(st - TIME * 0.05);
    // color.rg += st * 0.1;

    gl_FragColor = vec4(color, 1.0);
}
