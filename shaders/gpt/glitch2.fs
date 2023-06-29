/*
{
    "DESCRIPTION" : "Glitch Effect",
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
            "DEFAULT": 0.5,
            "MIN": 0.0,
            "MAX": 1.0,
            "DESCRIPTION": "Amount of glitch"
        },
        {
            "NAME": "speed",
            "TYPE": "float",
            "DEFAULT": 0.5,
            "MIN": 0.0,
            "MAX": 1.0,
            "DESCRIPTION": "Speed of glitch"
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
    vec4 color = texture2D(inputImage, uv);
    
    float t = TIME * speed;
    float offset = amount * (sin(t) + sin(t * 3.0) + sin(t * 7.0));
    vec2 offsetUV = vec2(uv.x + offset, uv.y);
    
    vec4 glitchColor = texture2D(inputImage, fract(offsetUV));
    
    gl_FragColor = mix(color, glitchColor, amount);
}
