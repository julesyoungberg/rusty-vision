/*
{
    "DESCRIPTION" : "Radial Blur Effect",
    "CREDIT" : "Created by ChatGPT",
    "CATEGORIES" : ["filter"],
    "INPUTS" : [
        {
            "NAME": "inputImage",
            "TYPE": "image",
            "DESCRIPTION": "Input image"
        },
        {
            "NAME": "blurAmount",
            "TYPE": "float",
            "DEFAULT": 0.2,
            "MIN": 0.0,
            "MAX": 1.0,
            "DESCRIPTION": "Amount of blur"
        },
        {
            "NAME": "center_x",
            "TYPE": "float",
            "DEFAULT": 0.5,
            "DESCRIPTION": "X Center point of the blur"
        },
        {
            "NAME": "center_y",
            "TYPE": "float",
            "DEFAULT": 0.5,
            "DESCRIPTION": "Y Center point of the blur"
        },
        {
            "NAME": "numPasses",
            "TYPE": "int",
            "DEFAULT": 8,
            "MIN": 1,
            "MAX": 20,
            "DESCRIPTION": "Number of blur passes"
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

void main()
{
    vec2 uv = gl_FragCoord.xy / RENDERSIZE.xy;
    vec2 p = uv;
    vec2 q = p - vec2(center_x, center_y);
    float r = length(q);
    float angle = atan(q.y, q.x);
    vec4 color = vec4(0.0, 0.0, 0.0, 0.0);
    float blurIncrement = blurAmount / float(numPasses);
    for(int i = 0; i < numPasses; i++) {
        float blurRadius = r + float(i) * blurIncrement;
        float x = blurRadius * cos(angle) + center_x;
        float y = blurRadius * sin(angle) + center_y;
        vec4 blurColor = texture2D(inputImage, vec2(x, y));
        color += blurColor;
    }
    color /= float(numPasses);
    gl_FragColor = color;
}
