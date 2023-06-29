/*
{
  "ISFVSN": "2",
  "DESCRIPTION": "A colorful and dynamic wave effect.",
  "CREDIT": "Based on the shader by Dave Hoskins",
  "CATEGORIES": [
    "generator"
  ],
  "INPUTS": [
    {
      "NAME": "speed",
      "DESCRIPTION": "The speed of the waves (1-10).",
      "DEFAULT": 5.0,
      "MIN": 1,
      "MAX": 10,
      "TYPE": "float"
    },
    {
      "NAME": "scale",
      "DESCRIPTION": "The scale of the waves (1-100).",
      "DEFAULT": 50.0,
      "MIN": 1,
      "MAX": 100,
      "TYPE": "float"
    },
    {
      "NAME": "color1",
      "DESCRIPTION": "The main color of the waves.",
      "DEFAULT": [0.5, 0.5, 0.5, 1.0],
      "TYPE": "color"
    },
    {
      "NAME": "color2",
      "DESCRIPTION": "The secondary color of the waves.",
      "DEFAULT": [1.0, 1.0, 1.0, 1.0],
      "TYPE": "color"
    }
  ]
}
*/

void main() {
    vec2 uv = (2.0 * isf_FragNormCoord.xy - 1.0) * RENDERSIZE / max(RENDERSIZE.x, RENDERSIZE.y);
    float freq = length(uv);
    float amp = 0.5 / freq * sin(freq * scale - TIME * speed);
    vec3 color = mix(color1.rgb, color2.rgb, (amp + 1.0) / 2.0);
    gl_FragColor = vec4(color, 1.0);
}