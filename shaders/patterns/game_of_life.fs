/*{
    "DESCRIPTION": "The game of life",
    "CREDIT": "by jules youngberg",
    "ISFVSN": "2.0",
    "CATEGORIES": ["GENERATOR"],
    "INPUTS": [
        {
            "NAME": "inputImage",
            "TYPE": "image"
        },
        {
            "NAME": "initEvent",
            "TYPE": "event"
        },
        {
            "NAME": "blurAmount",
            "TYPE": "float"
        }
    ],
    "PASSES": [
        {
            "TARGET": "buffer",
            "PERSISTENT": true,
            "FLOAT": true
        }
    ]
}*/

void main() {
    vec4 srcPixel = vec4(0.0, 0.0, 0.0, 1.0);
    gl_FragColor = (initEvent == true) ? vec4(1.0, 1.0, 1.0, 1.0) : srcPixel;
}
