/*{
    "DESCRIPTION": "Tap motion effect.",
    "CREDIT": "by julesyoungberg",
    "ISFVSN": "2.0",
    "CATEGORIES": [ "Blur" ],
    "INPUTS": [
        {
            "NAME": "inputImage",
            "TYPE": "image"
        },
        {
            "NAME": "tap",
            "TYPE": "event"
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
    gl_FragColor = tap ? IMG_THIS_PIXEL(inputImage) : IMG_THIS_PIXEL(prev_frame);
}
