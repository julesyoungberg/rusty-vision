/*
{
    "DESCRIPTION" : "Random Movement Effect",
    "CREDIT" : "Created by ChatGPT",
    "CATEGORIES" : ["transition"],
    "INPUTS" : [
        {
            "NAME": "inputImage",
            "TYPE": "image",
            "DESCRIPTION": "Input image"
        },
        {
            "NAME": "numCopies",
            "TYPE": "float",
            "DEFAULT": 20.0,
            "MIN": 1.0,
            "MAX": 50.0,
            "DESCRIPTION": "Number of image copies"
        },
        {
            "NAME": "maxMovement",
            "TYPE": "float",
            "DEFAULT": 0.2,
            "MIN": 0.0,
            "MAX": 1.0,
            "DESCRIPTION": "Maximum amount of movement"
        }
    ],
    "OUTPUTS" : [
        {
            "NAME" : "outputImage",
            "TYPE" : "image",
            "DESCRIPTION" : "Output image"
        }
    ]
}
*/

float random(vec2 st) {
    return fract(sin(dot(st.xy, vec2(12.9898,78.233))) * 43758.5453123);
}

void main()
{
    vec2 uv = gl_FragCoord.xy / RENDERSIZE.xy;
    vec4 color = vec4(0.0, 0.0, 0.0, 0.0);
    for (float i = 0.0; i < numCopies; i += 1.0) {
        vec2 offset = vec2(random(vec2(i, 0.0)) - 0.5, random(vec2(0.0, i)) - 0.5) * maxMovement;
        vec2 sampleUV = uv + offset;
        vec4 blurColor = texture2D(inputImage, sampleUV);
        color += blurColor;
    }
    color /= numCopies;
    color.a = 1.0;
    gl_FragColor = color;
}
