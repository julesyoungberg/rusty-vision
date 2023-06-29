/*
{
  "ISFVSN": "2",
  "DESCRIPTION": "Adds a magical sparkle effect to a video.",
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
      "NAME": "sparkleColor",
      "DESCRIPTION": "The color of the sparkles.",
      "DEFAULT": [1.0, 1.0, 1.0, 1.0],
      "TYPE": "color"
    },
    {
      "NAME": "sparkleSize",
      "DESCRIPTION": "The size of the sparkles (0-1).",
      "DEFAULT": 0.05,
      "MIN": 0,
      "MAX": 1,
      "TYPE": "float"
    },
    {
      "NAME": "sparkleBrightness",
      "DESCRIPTION": "The brightness of the sparkles (0-1).",
      "DEFAULT": 0.5,
      "MIN": 0,
      "MAX": 1,
      "TYPE": "float"
    },
    {
      "NAME": "sparkleSpeed",
      "DESCRIPTION": "The speed of the sparkles (0-1).",
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
    float time = TIME;
    vec2 sparklePos = vec2(mod(uv.x + time * sparkleSpeed, 1.0), mod(uv.y + time * sparkleSpeed, 1.0));
    float sparkle = smoothstep(1.0 - sparkleSize, 1.0, length(uv - sparklePos));
    sparkle = pow(sparkle, 2.0);
    color += sparkle * sparkleBrightness * sparkleColor;
    gl_FragColor = color;
}