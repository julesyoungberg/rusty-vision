/*{
    "DESCRIPTION": "",
    "CREDIT": "by julesyoungberg",
    "ISFVSN": "2.0",
    "CATEGORIES": [ "GENERATOR" ],
    "INPUTS": []
}*/

#define PI 3.14159265359

float Xor(float a, float b) { return a * (1.0 - b) + b * (1.0 - a); }

void main() {
    vec2 st = isf_FragNormCoord * 2.0 - 1.0;
    st.y *= RENDERSIZE.y / RENDERSIZE.x;

    float angle = PI / 4.0; // + length(st) * sin(TIME * 0.1);
    float c = cos(angle);
    float s = sin(angle);
    st *= mat2(c, -s, s, c);
    st *= 20.0;

    vec3 color = vec3(0);

    vec2 gv = fract(st) - 0.5;
    vec2 id = floor(st);
    float m = 0;
    float t = TIME * -1.5;

    for (float y = -1.0; y <= 1.0; y++) {
        for (float x = -1.0; x <= 1.0; x++) {
            vec2 offset = vec2(x, y);
            vec2 lid = id + offset;
            vec2 lgv = gv - offset;

            angle = TIME;
            c = cos(angle);
            s = sin(angle);
            lgv *= mat2(c, -s, s, c);

            float center_dist = length(lid) * 0.3;
            float r = mix(0.5, 1.5, sin(t + center_dist) * 0.5 + 0.5);
            float circle_dist = length(lgv);
            m = Xor(smoothstep(r, r * 0.95, circle_dist), m);
        }
    }

    // color += m;
    color =
        sin(TIME * 0.5 + m * vec3(0.1, PI * 1.8, PI * 4.7) + length(gv)) * 0.5 +
        0.5;

    gl_FragColor = vec4(color, 1);
}
