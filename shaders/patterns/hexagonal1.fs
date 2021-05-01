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

vec3 hsv2rgb(vec3 c) {
    vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
    return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}

float hex_dist(in vec2 p) {
    p = abs(p);
    return max(dot(p, normalize(vec2(1.0, sqrt(3)))), p.x);
}

vec4 hex_coords(in vec2 st) {
    vec2 r = vec2(1, sqrt(3));
    vec2 h = r * 0.5;

    vec2 a = mod(st, r) - h;
    vec2 b = mod(st - h, r) - h;

    vec2 gv = length(a) < length(b) ? a : b;

    float x = atan(gv.x, gv.y);
    float y = 0.5 - hex_dist(gv);
    vec2 id = st - gv;

    return vec4(x, y, id);
}

void main() {
    vec2 st = isf_FragNormCoord * 2.0 - 1.0;
    st.y *= RENDERSIZE.y / RENDERSIZE.x;

    vec3 color = vec3(0);

    st *= 10.0;

    vec4 coords = hex_coords(st);
    vec2 gv = coords.xy;
    vec2 id = coords.zw;

    float i = dot(id, id);

    float intensity =
        log(IMG_NORM_PIXEL(fft_texture, vec2(fract(i * 0.1), 0)).x * 3.0 + 1.0);

    float d = smoothstep(0.01, 0.03, gv.y * sin(i + TIME)); // * intensity * 0.5
    // color += c;

    color = d * hsv2rgb(vec3(sin(i + TIME * 0.1), 1, 1)).zxy *
            log(intensity * 10.0);
    // color = mix(vec3(0), color, smoothstep(0.05, 0.06, m_edge_dist));

    gl_FragColor = vec4(color, 1);
}
