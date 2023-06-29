/*
{
  "ISFVSN": "2",
  "DESCRIPTION": "A shader effect inspired by https://www.shadertoy.com/view/Xsf3Rn",
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
      "NAME": "intensity",
      "DESCRIPTION": "The intensity of the effect (0-1).",
      "DEFAULT": 0.5,
      "MIN": 0,
      "MAX": 1,
      "TYPE": "float"
    },
    {
      "NAME": "size",
      "DESCRIPTION": "The size of the effect (0-1).",
      "DEFAULT": 0.5,
      "MIN": 0,
      "MAX": 1,
      "TYPE": "float"
    },
    {
      "NAME": "speed",
      "DESCRIPTION": "The speed of the effect (0-1).",
      "DEFAULT": 0.5,
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
    vec2 pos = vec2(uv.x + TIME * speed * 0.1, uv.y);
    float wave = size * sin(pos.x * 10.0 + pos.y * 10.0 + TIME * 10.0);
    float mask = smoothstep(0.0, 0.001, abs(wave - uv.y));
    color.rgb += intensity * mask;
    gl_FragColor = color;
}