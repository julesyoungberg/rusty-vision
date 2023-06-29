/*{
    "DESCRIPTION": "Motion blur.",
    "CREDIT": "by zoidberg",
    "ISFVSN": "2.0",
    "CATEGORIES": [ "Blur" ],
    "INPUTS": [
        {
            "NAME": "inputImage",
            "TYPE": "image"
        },
        {
            "NAME": "amount",
            "TYPE": "float"
        }
    ],
    "PASSES": [
        {
            "TARGET": "lastFrame",
            "PERSISTENT": true,
            "FLOAT": true
        }
    ]
}*/

void main() {
    vec4 freshPixel = IMG_THIS_PIXEL(inputImage);
    vec4 stalePixel = IMG_THIS_PIXEL(lastFrame);
    gl_FragColor = fract(stalePixel + freshPixel * amount);
}
