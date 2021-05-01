/*{
    "DESCRIPTION": "",
    "CREDIT": "by julesyoungberg",
    "ISFVSN": "2.0",
    "CATEGORIES": [ "GENERATOR" ],
    "INPUTS": []
}*/

#define PI 3.14159265359
#define TAU 6.28318530718

const vec2 s = vec2(1, 1.7320508);

// shane's hexagonal tiling (https://www.shadertoy.com/view/llSyDh)
vec4 get_hex(vec2 p) {
    vec4 hc = floor(vec4(p, p - vec2(0.5, 1)) / s.xyxy) + 0.5;
    vec4 h = vec4(p - hc.xy * s, p - (hc.zw + 0.5) * s);
    return (dot(h.xy, h.xy) < dot(h.zw, h.zw))
               ? vec4(h.xy, hc.xy)
               : vec4(h.zw, hc.zw + vec2(0.5, 1));
}

float Xor(float a, float b) { return a * (1.0 - b) + b * (1.0 - a); }

void main() {
    vec2 st = isf_FragNormCoord * 2.0 - 1.0;
    st.y *= RENDERSIZE.y / RENDERSIZE.x;

    vec3 color = vec3(0);

    for (float i = 0; i < 3; i++) {
        vec2 p = st;

        float shift =
            sin(i * PI + TIME * (i + 0.1) * 1.3 + length(p) * 15.0) * 0.005;
        p += shift;
        p *= 15.0;
        p = mix(p, p * (1.0 - length(st)), sin(TIME * 0.7) * 0.5 + 0.5);

        vec4 hex = get_hex(p);
        vec2 gv = hex.xy;
        vec2 id = hex.zw;

        float m = 0.0;
        float t = TIME;

        for (float j = 0.0; j <= 6.0; j++) {
            vec2 offset = vec2(0);
            if (j < 6.0) {
                float angle = j * TAU / 6.0;
                float si = sin(angle);
                float co = cos(angle);
                offset = vec2(1.0, 0.0) * mat2(co, -si, si, co);
            }

            vec2 other_id = get_hex(p + offset).zw;

            float d = length(gv - offset);
            float dist = length(other_id * s) * 0.3;
            float r = mix(0.3, 1.0, sin(dist - t) * 0.5 + 0.5);
            m = Xor(m, smoothstep(r, r * 0.95, d));
        }

        color[int(i)] += m;
    }

    gl_FragColor = vec4(color, 1);
}
