/*{
    "DESCRIPTION": "demonstrates the use of float-type inputs",
    "CREDIT": "by zoidberg",
    "ISFVSN": "2.0",
    "CATEGORIES": [
        "TEST-GLSL FX"
    ],
    "INPUTS": [
        {
            "NAME": "inputImage",
            "TYPE": "image"
        },
        {
            "NAME": "level",
            "TYPE": "long",
            "DEFAULT": 0,
            "MIN": 0,
            "MAX": 1000
        }
    ]
}*/

void main() {
    // vec4 srcPixel = IMG_THIS_PIXEL(inputImage);
    // float luma = (srcPixel.r + srcPixel.g + srcPixel.b) / 3.0;
    // vec4 dstPixel = (luma > level) ? srcPixel : vec4(0, 0, 0, 1);
    // gl_FragColor = dstPixel;
    gl_FragColor = vec4(float(level) / 1000.0);
}
