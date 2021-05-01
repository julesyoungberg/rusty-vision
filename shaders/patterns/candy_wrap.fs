/*{
    "DESCRIPTION": "",
    "CREDIT": "by julesyoungberg",
    "ISFVSN": "2.0",
    "CATEGORIES": [ "GENERATOR" ],
    "INPUTS": [
        {
            "NAME": "scale_factor",
            "TYPE": "float",
            "MIN": 1.0,
            "MAX": 100.0,
            "DEFAULT": 50.0
        },
        {
            "NAME": "radius_size",
            "TYPE": "float",
            "MIN": 0.1,
            "MAX": 10.0,
            "DEFAULT": 2.5
        },
        {
            "NAME": "gap_size",
            "TYPE": "float",
            "MIN": 0.1,
            "MAX": 2.0,
            "DEFAULT": 0.9
        },
        {
            "NAME": "speed",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 2.0,
            "DEFAULT": 1.3
        },
        {
            "NAME": "loops",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 100.0,
            "DEFAULT": 50.0
        },
        {
            "NAME": "modulation",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 10.0,
            "DEFAULT": 2.5
        },
        {
            "NAME": "modulation_scale",
            "TYPE": "point2D",
            "MIN": [0.0, 0.0],
            "MAX": [1.0, 1.0],
            "DEFAULT": [0.3, 0.2]
        },
        {
            "NAME": "time_scale1",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 0.1,
            "DEFAULT": 0.001
        },
        {
            "NAME": "time_scale2",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 0.1,
            "DEFAULT": 0.011
        },
        {
            "NAME": "time_scale3",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 0.1,
            "DEFAULT": 0.009
        },
        {
            "NAME": "color_shift1",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 3.14,
            "DEFAULT": 0.0
        },
        {
            "NAME": "color_shift2",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 3.14,
            "DEFAULT": 1.5
        },
        {
            "NAME": "color_shift3",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 3.14,
            "DEFAULT": 3.14
        }
    ]
}*/

// based on CandyWrap by mojovideotech
// https://editor.isf.video/shaders/5e7a802d7c113618206dec38

void main() {
    vec2 st = isf_FragNormCoord * 2.0 - 1.0;
    st *= RENDERSIZE;

    vec3 color = vec3(0.0);

    float scale = RENDERSIZE.y / scale_factor;
    float radius = RENDERSIZE.x * radius_size;
    float gap = scale * gap_size;

    float d = length(st);
    float t = TIME * speed;

    // modulate the distance
    d += scale * modulation *
         (sin(st.y * modulation_scale.x / scale + t) *
          sin(st.x * modulation_scale.y / scale + t * 0.5));
    float v = mod(d + radius / (loops * 2.0), radius / loops);
    v = abs(v - radius / (loops * 2.0));
    v = clamp(v - gap, 0.0, 1.0);
    d /= radius;

    vec3 m = fract((d - 1.0) *
                   vec3(loops * sin(TIME * time_scale1 + color_shift1),
                        loops * sin(TIME * time_scale2 + color_shift2),
                        loops * sin(TIME * time_scale3 + color_shift3)) *
                   0.5);
    color = m * v;

    gl_FragColor = vec4(color, 1.0);
}
