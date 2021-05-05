/*{
    "DESCRIPTION": "Stop motion effect.",
    "CREDIT": "by julesyoungberg",
    "ISFVSN": "2.0",
    "CATEGORIES": [ "Blur" ],
    "INPUTS": [
        {
            "NAME": "inputImage",
            "TYPE": "image"
        },
        {
            "NAME": "speed",
            "TYPE": "float",
            "MIN": 1.0,
            "MAX": 20.0,
            "DEFAULT": 15.0
        }
    ],
    "PASSES": [
        {
            "TARGET": "prev_frame",
            "PERSISTENT": true,
            "FLOAT": true
        }
    ]
}*/

void main() {
    vec4 fresh_pixel = IMG_THIS_PIXEL(inputImage);
    vec4 stale_pixel = IMG_THIS_PIXEL(prev_frame);
    float t = step(0.9, sin(TIME * speed) * 0.5 + 0.5);
    gl_FragColor = mix(stale_pixel, fresh_pixel, t);
}
