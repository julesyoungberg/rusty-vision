/*
{
    "DESCRIPTION" : "Noise/Directional Blur Effect",
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
            "DEFAULT": 0.05,
            "MIN": 0.0,
            "MAX": 1.0,
            "DESCRIPTION": "Amount of blur"
        },
        {
            "NAME": "noiseAmount",
            "TYPE": "float",
            "DEFAULT": 0.05,
            "MIN": 0.0,
            "MAX": 1.0,
            "DESCRIPTION": "Amount of noise"
        },
        {
            "NAME": "direction_x",
            "TYPE": "float",
            "DEFAULT": 0.5,
            "DESCRIPTION": "X Direction of the blur"
        },
        {
            "NAME": "direction_y",
            "TYPE": "float",
            "DEFAULT": 0.5],
            "DESCRIPTION": "Y Direction of the blur"
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
    vec2 p = -1.0 + 2.0 * uv;
    vec2 offset = vec2(texture2D(inputImage, uv).r * 2.0 - 1.0) * noiseAmount;
    vec4 color = vec4(0.0, 0.0, 0.0, 0.0);
    float blurRadius = 0.0;
    vec2 direction = vec2(direction_x, direction_y);
    for(float i = 0.0; i < 10.0; i += 1.0) {
        vec2 sampleUV = uv + offset + i * blurAmount * direction;
        vec4 blurColor = texture2D(inputImage, sampleUV);
        color += blurColor;
        blurRadius += length(blurColor.rgb);
    }
    color /= 10.0;
    blurRadius /= 10.0;
    color.a = 1.0;
    gl_FragColor = color;
}
