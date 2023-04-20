/*{
    "DESCRIPTION": "Fade",
    "CREDIT": "by julesyoungberg",
    "ISFVSN": "2.0",
    "CATEGORIES": [ "Utilities" ],
    "INPUTS": [
        {
            "NAME": "inputImage",
            "TYPE": "image"
        }
    ],
    "PASSES": []
}*/

void main() {
    vec4 freshPixel = IMG_THIS_PIXEL(inputImage);
    gl_FragColor = vec4(step(0.1, (freshPixel.r + freshPixel.g + freshPixel.b) / 3.0));
}
