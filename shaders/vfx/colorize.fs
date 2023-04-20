/*{
    "DESCRIPTION": "Colorize",
    "CREDIT": "by julesyoungberg",
    "ISFVSN": "2.0",
    "CATEGORIES": [ "Utilities" ],
    "INPUTS": [
        {
            "NAME": "inputImage",
            "TYPE": "image"
        },
        {
            "NAME": "frontColor",
            "TYPE": "color",
            "DEFAULT": [
                1.0,
                1.0,
                1.0,
                1.0
            ]
        },
        {
            "NAME": "backColor",
            "TYPE": "color",
            "DEFAULT": [
                0.0,
                0.0,
                0.0,
                1.0
            ]
        },
        {
            "NAME": "frontBrightness",
            "TYPE": "float",
            "DEFAULT": 1.0,
            "MIN": 0.0,
            "MAX": 1.0
        },
        {
            "NAME": "backBrightness",
            "TYPE": "float",
            "DEFAULT": 1.0,
            "MIN": 0.0,
            "MAX": 1.0
        }
    ],
    "PASSES": []
}*/

void main() {
    vec4 pixel = IMG_THIS_PIXEL(inputImage);
    float t = (pixel.r + pixel.g + pixel.b) / 3.0;
    gl_FragColor = mix(backColor * sqrt(backBrightness), frontColor * sqrt(frontBrightness), t);
}
