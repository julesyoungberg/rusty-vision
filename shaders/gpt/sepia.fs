/*
{
    "DESCRIPTION" : "Sepia Effect",
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
            "DEFAULT": 0.5,
            "MIN": 0.0,
            "MAX": 1.0,
            "DESCRIPTION": "Intensity of the effect"
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
  vec4 color = texture2D(inputImage,gl_FragCoord.xy/RENDERSIZE.xy);
  float gray = dot(color.rgb, vec3(0.299, 0.587, 0.114));
  vec3 sepia = vec3(1.2, 1.0, 0.8);
  color.rgb = mix(vec3(gray), sepia, intensity);
  gl_FragColor = color;
}
