/*{
    "DESCRIPTION": "Stop motion effect.",
    "CREDIT": "by julesyoungberg",
    "ISFVSN": "2.0",
    "CATEGORIES": [ "FX" ],
    "INPUTS": [
        {
            "NAME": "input_image",
            "TYPE": "image"
        },
        {
            "NAME": "speed",
            "TYPE": "float",
            "MAX": 10.0
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
    vec4 fresh_pixel = IMG_THIS_PIXEL(input_image);
    vec4 stale_pixel = IMG_THIS_PIXEL(prev_frame);
    gl_FragColor =
        mix(stale_pixel, fresh_pixel, float(mod(time, speed) == 0.0));
}
