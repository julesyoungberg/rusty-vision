/*{
        "DESCRIPTION": "",
        "CREDIT": "by zoidberg",
        "ISFVSN": "2.0",
        "CATEGORIES": [ "TEST-GLSL FX" ],
        "INPUTS": [
            {
                "NAME": "inputImage",
                "TYPE": "image"
            }
        ],
        "PASSES": [
            {
                "TARGET":"bufferVariableNameA",
                "WIDTH": "$WIDTH/16.0",
                "HEIGHT": "$HEIGHT/16.0"
            },
            {}
        ]
}*/

void main() {
    //	first pass: read the "inputImage"- remember, we're drawing to the
    // persistent buffer "bufferVariableNameA" on the first pass
    if (PASSINDEX == 0) {
        gl_FragColor = IMG_THIS_NORM_PIXEL(inputImage);
    }
    //	second pass: read from "bufferVariableNameA".  output looks chunky and
    // low-res.
    else if (PASSINDEX == 1) {
        gl_FragColor = IMG_THIS_NORM_PIXEL(bufferVariableNameA);
    }
}
