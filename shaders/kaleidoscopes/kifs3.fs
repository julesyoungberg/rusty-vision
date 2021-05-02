/*{
    "DESCRIPTION": "Audio reaactive glitch effects",
    "CREDIT": "by julesyoungberg",
    "ISFVSN": "2.0",
    "CATEGORIES": [ "FX" ],
    "INPUTS": [
        {
            "NAME": "input_image",
            "TYPE": "image"
        }
    ]
}*/

#define PI 3.14159265359
#define PHI 1.6180339887

vec2 N(float angle) { return vec2(sin(angle), cos(angle)); }

float sdBox(in vec2 p, in vec2 b) {
    vec2 d = abs(p) - b;
    return length(max(d, 0.0)) + min(max(d.x, d.y), 0.0);
}

float square(in vec2 p, in vec2 b) {
    float angle = PI * 0.25 * TIME * -0.5;
    float c = cos(angle);
    float s = sin(angle);
    return sdBox(p * mat2(c, -s, s, c), b);
}

vec3 image_color(in vec2 coord) {
    return IMG_NORM_PIXEL(input_image, fract(coord)).rgb;
}

void main() {
    vec2 st = isf_FragNormCoord * 2.0 - 1.0;
    st.y *= RENDERSIZE.y / RENDERSIZE.x;
    st *= 3.0;
    vec3 color = vec3(0);
    vec2 size = vec2(0.5);

    float scale = 1.0;
    float dist = 100.0;

    for (int i = 0; i < 10; i++) {
        float angle = TIME * 0.01 + i;
        float c = cos(angle);
        float s = sin(angle);
        st *= mat2(c, -s, s, c);

        bool even = mod(i, 2) == 0;
        dist = min(dist, square(st, size) * scale);

        if (even) {
            st.x = abs(st.x);
        } else {
            st.y = abs(st.y);
        }

        st *= PHI;
        scale /= PHI;

        if (even) {
            st.x -= PHI * sqrt(2);
        } else {
            st.y -= PHI * sqrt(2);
        }
    }

    st *= scale;
    color = image_color(st - TIME * 0.3);
    // color += 1.0 - sign(dist);

    gl_FragColor = vec4(color, 1.0);
}
