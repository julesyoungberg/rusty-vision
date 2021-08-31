/*{
    "DESCRIPTION": "",
    "CREDIT": "by julesyoungberg",
    "ISFVSN": "2.0",
    "CATEGORIES": [ "GENERATOR" ],
    "INPUTS": [
        {
            "NAME": "color1",
            "TYPE": "color",
            "DEFAULT": [
                1.0,
                1.0,
                1.0,
                1.0
            ]
        },
        {
            "NAME": "scale",
            "TYPE": "float",
            "MIN": 1.0,
            "MAX": 20.0,
            "DEFAULT": 10.0
        },
        {
            "NAME": "pan_speed",
            "TYPE": "float",
            "MIN": -0.5,
            "MAX": 0.5,
            "DEFAULT": 0.1
        },
        {
            "NAME": "speed",
            "TYPE": "float",
            "MIN": -2.0,
            "MAX": 2.0,
            "DEFAULT": 1.0
        }
    ]
}*/

float hash21(vec2 p) {
    p = fract(p * vec2(234.34, 435.345));
    p += dot(p, p);
    return fract(p.x * p.y);
}

vec3 truchet_pattern(vec2 p, float width) {
    vec2 gv = fract(p) - 0.5;
    vec2 id = floor(p);

    float n = hash21(id);
    if (n < 0.5)
        gv.x *= -1.0;

    vec2 c_uv = gv - 0.5 * sign(gv.x + gv.y + 0.001);
    float d = length(c_uv);

    float mask = smoothstep(0.01, -0.01, abs(d - 0.5) - width);

    float angle = atan(c_uv.x, c_uv.y);
    float checker = mod(id.x + id.y, 2.0) * 2.0 - 1.0;

    // float flow = sin(angle * checker * 10.0 + 2.0 * TIME);
    float x = fract(checker * angle / 1.57 + TIME * speed);
    float y = (d - (0.5 - width)) / (width * 2.0);
    y = abs(y - 0.5) * 2.0; // mirror
    // if (n < 0.5 ^^ checker > 0.0) y = 1.0 - y; // continuous
    return vec3(x, y, mask);
}

void main() {
    vec2 st = isf_FragNormCoord * 2.0 - 1.0;
    st.y *= RENDERSIZE.y / RENDERSIZE.x;

    vec3 color = vec3(0.0);

    vec2 og = st;

    st += TIME * pan_speed;
    st *= scale;

    vec3 truchet = truchet_pattern(st, 0.2 * (1.0 - length(og)));
    vec2 t_uv = truchet.xy;
    float mask = truchet.z;
    float y = t_uv.y;
    t_uv.y *= 0.2;
    t_uv.x -= 0.5;
    t_uv *= 2.0;
    t_uv.x = fract(t_uv.x) - 0.5;

    color += mask * smoothstep(0.2, 0.21, abs(length(t_uv * vec2(1.0, 1.5)))) *
             (1.0 - y);

    color *= color1.rgb;

    // if (gv.x > 0.48 || gv.y > 0.48) color = vec3(1, 0, 0);

    gl_FragColor = vec4(color, 1);
}
