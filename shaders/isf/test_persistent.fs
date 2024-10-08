/*{
    "DESCRIPTION": "",
    "CREDIT": "by zoidberg",
    "ISFVSN": "2.0",
    "CATEGORIES": [ "TEST-GLSL FX" ],
    "INPUTS": [
        {
            "NAME": "inputImage",
            "TYPE": "image"
        },
        {
            "NAME": "blurAmount",
            "TYPE": "float"
        }
    ],
    "PASSES": [
        {
            "TARGET": "bufferVariableNameA",
            "PERSISTENT": true,
            "FLOAT": true
        }
    ]
}*/

void main() {
    vec4 freshPixel = IMG_THIS_PIXEL(inputImage);
    vec4 stalePixel = IMG_THIS_PIXEL(bufferVariableNameA);
    gl_FragColor = mix(freshPixel, stalePixel, blurAmount);
}
