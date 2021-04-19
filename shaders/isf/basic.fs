/*{
        "DESCRIPTION": "demonstrates the use of float-type inputs",
        "CREDIT": "by zoidberg",
        "ISFVSN": "2.0",
        "CATEGORIES": [
                "TEST-GLSL FX"
        ],
        "INPUTS": []
}*/

void main() {
    vec3 color = vec3(0.0);

    color = vec3(isf_FragNormCoord, sin(TIME) * 0.5 + 0.5);

    gl_FragColor = vec4(color, 1.0);
}
