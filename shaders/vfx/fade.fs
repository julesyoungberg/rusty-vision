/*{
    "DESCRIPTION": "Fade",
    "CREDIT": "by julesyoungberg",
    "ISFVSN": "2.0",
    "CATEGORIES": [ "Utilities" ],
    "INPUTS": [
        {
            "NAME": "inputImage",
            "TYPE": "image"
        },
        {
            "NAME": "fade",
            "TYPE": "float"
        }
    ],
    "PASSES": []
}*/

void main() {
    vec4 freshPixel = IMG_THIS_PIXEL(inputImage);
    gl_FragColor = vec4(freshPixel.rgb * fade, freshPixel.a);
}
