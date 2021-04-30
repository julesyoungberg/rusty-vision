/*{
    "DESCRIPTION": "Audio reaactive glitch effects",
    "CREDIT": "by julesyoungberg",
    "ISFVSN": "2.0",
    "CATEGORIES": [ "FX" ],
    "INPUTS": [
        {
            "NAME": "inputImage",
            "TYPE": "image"
        },
        {
            "NAME": "fftImage",
            "TYPE": "audioFFT"
        }
    ]
}*/

void main() {
    vec3 color = vec3(0);

    color = IMG_THIS_PIXEL(inputImage).rgb;

    float t = floor(TIME * 30.0);

    gl_FragColor = vec4(color, 1.0);
}
