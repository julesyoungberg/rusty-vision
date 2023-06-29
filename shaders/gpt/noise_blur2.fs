/*
{
  "ISFVSN": "2",
  "DESCRIPTION": "A distortion effect inspired by https://www.shadertoy.com/view/4dlGDN.",
  "CREDIT": "",
  "CATEGORIES": [
    "filter"
  ],
  "INPUTS": [
    {
      "NAME": "inputImage",
      "DESCRIPTION": "The input image.",
      "TYPE": "image"
    },
    {
      "NAME": "distortionAmount",
      "DESCRIPTION": "The amount of distortion (0-1).",
      "DEFAULT": 0.1,
      "MIN": 0,
      "MAX": 1,
      "TYPE": "float"
    }
  ]
}
*/

void main() {
    vec2 uv = isf_FragNormCoord.xy;
    vec4 color = texture(inputImage, uv);
    vec2 distortion = vec2(
      sin(uv.x * 10.0 + distortionAmount * sin(uv.y * 20.0)) * distortionAmount,
      sin(uv.y * 10.0 + distortionAmount * sin(uv.x * 20.0)) * distortionAmount
    );
    vec2 distortedUv = uv + distortion;
    vec4 distortedColor = texture(inputImage, distortedUv);
    color.rgb = distortedColor.rgb;
    gl_FragColor = color;
}