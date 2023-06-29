/*
{
    "DESCRIPTION" : "Organic Ripple Effect",
    "CREDIT" : "Created by ChatGPT",
    "CATEGORIES" : ["filter"],
    "INPUTS" : [
        {
            "NAME": "inputImage",
            "TYPE": "image",
            "DESCRIPTION": "Input image"
        },
        {
            "NAME": "amount",
            "TYPE": "float",
            "DEFAULT": 0.1,
            "MIN": 0.0,
            "MAX": 1.0,
            "DESCRIPTION": "Amount of ripple"
        },
        {
            "NAME": "frequency",
            "TYPE": "float",
            "DEFAULT": 10.0,
            "MIN": 1.0,
            "MAX": 50.0,
            "DESCRIPTION": "Frequency of the ripple"
        },
        {
            "NAME": "speed",
            "TYPE": "float",
            "DEFAULT": 0.5,
            "MIN": 0.0,
            "MAX": 1.0,
            "DESCRIPTION": "Speed of the ripple"
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

void main() {
    vec2 uv = gl_FragCoord.xy / RENDERSIZE.xy;
    vec2 p = uv;
    vec2 q = p + vec2(0.0, 0.1 * amount);
    vec2 r = p + vec2(0.1 * amount, 0.0);
    float sineWave = sin(p.x * frequency + TIME * speed) * amount;
    vec4 color = texture2D(inputImage, fract(uv + vec2(sineWave, 0.0)));
    vec4 qColor = texture2D(inputImage, fract(q + vec2(sineWave, 0.0)));
    vec4 rColor = texture2D(inputImage, fract(r + vec2(sineWave, 0.0)));
    float average = (color.r + qColor.g + rColor.b) / 3.0;
    vec4 outputColor = vec4(average, average, average, 1.0);
    gl_FragColor = outputColor;
}
