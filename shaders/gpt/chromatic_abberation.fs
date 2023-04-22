/*
{
    "DESCRIPTION" : "Chromatic Aberration Effect",
    "CREDIT" : "Created by ChatGPT",
    "CATEGORIES" : ["filter"],
    "INPUTS" : [
        {
            "NAME": "inputImage",
            "TYPE": "image",
            "DESCRIPTION": "Input image"
        },
        {
            "NAME": "intensity",
            "TYPE": "float",
            "DEFAULT": 0.1,
            "MIN": 0.0,
            "MAX": 1.0,
            "DESCRIPTION": "Intensity of the effect"
        },
        {
            "NAME": "offsetX",
            "TYPE": "float",
            "DEFAULT": 0.005,
            "MIN": 0.0,
            "MAX": 0.1,
            "DESCRIPTION": "Horizontal offset"
        },
        {
            "NAME": "offsetY",
            "TYPE": "float",
            "DEFAULT": 0.01,
            "MIN": 0.0,
            "MAX": 0.1,
            "DESCRIPTION": "Vertical offset"
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
  
  vec4 rColor = texture2D(inputImage, uv + vec2(offsetX, 0.0));
  vec4 bColor = texture2D(inputImage, uv - vec2(offsetX, 0.0));
  vec4 gColor = texture2D(inputImage, uv + vec2(0.0, offsetY));
  
  color.r = mix(color.r, rColor.r, intensity);
  color.b = mix(color.b, bColor.b, intensity);
  color.g = mix(color.g, gColor.g, intensity);
  
  gl_FragColor = color;
}
