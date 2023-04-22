/*
{
    "DESCRIPTION" : "Organic Blooming Effect",
    "CREDIT" : "Created by ChatGPT",
    "CATEGORIES" : ["filter"],
    "INPUTS" : [
        {
            "NAME": "inputImage",
            "TYPE": "image",
            "DESCRIPTION": "Input image"
        },
        {
            "NAME": "blur",
            "TYPE": "float",
            "DEFAULT": 0.1,
            "MIN": 0.0,
            "MAX": 1.0,
            "DESCRIPTION": "Amount of blur"
        },
        {
            "NAME": "threshold",
            "TYPE": "float",
            "DEFAULT": 0.5,
            "MIN": 0.0,
            "MAX": 1.0,
            "DESCRIPTION": "Threshold for blooming"
        },
        {
            "NAME": "power",
            "TYPE": "float",
            "DEFAULT": 2.0,
            "MIN": 0.0,
            "MAX": 5.0,
            "DESCRIPTION": "Power of the bloom effect"
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

    // Compute luminance of color
    float luminance = dot(color.rgb, vec3(0.299, 0.587, 0.114));
    
    // Compute bloom mask
    float mask = smoothstep(threshold, threshold + blur, luminance);
    
    // Compute bloom color
    vec4 bloomColor = vec4(1.0, 1.0, 1.0, 1.0) * pow(mask, power);
    
    // Blend color and bloom color
    vec4 outputColor = mix(color, bloomColor, mask);
    
    gl_FragColor = outputColor;
}
