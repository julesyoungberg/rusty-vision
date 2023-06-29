/*{
  "DESCRIPTION": "Adds a bokeh effect to a video.",
  "CREDIT": "Based on the bokeh shader by Patricio Gonzalez Vivo",
  "ISFVSN": "2",
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
      "NAME": "focus",
      "DESCRIPTION": "The distance from the camera to the subject in focus (0-1).",
      "MIN": 0,
      "MAX": 1,
      "DEFAULT": 0.5,
      "TYPE": "float"
    },
    {
      "NAME": "blurSize",
      "DESCRIPTION": "The size of the blur effect (0-1).",
      "MIN": 0,
      "MAX": 1,
      "DEFAULT": 0.1,
      "TYPE": "float"
    },
    {
      "NAME": "bokehSize",
      "DESCRIPTION": "The size of the bokeh effect (0-1).",
      "MIN": 0,
      "MAX": 1,
      "DEFAULT": 0.025,
      "TYPE": "float"
    },
    {
      "NAME": "bokehBrightness",
      "DESCRIPTION": "The brightness of the bokeh effect (0-1).",
      "MIN": 0,
      "MAX": 1,
      "DEFAULT": 0.5,
      "TYPE": "float"
    }
  ]
}*/

float rand(vec2 p) {
    return fract(sin(dot(p.xy, vec2(12.9898, 78.233))) * 43758.5453);
}

void main() {
    vec2 uv = isf_FragNormCoord.xy;
    vec4 color = texture(inputImage, uv);
    float depth = color.r;
    float aperture = 0.025;
    float maxBlur = 0.05;
    float focalRange = 0.025;
    float focalDistance = focus - depth;
    float blurAmount = abs(focalDistance) / focalRange;
    blurAmount = smoothstep(0.0, 1.0, blurAmount);
    blurAmount = pow(blurAmount, 2.0);
    float bokehAmount = max(blurAmount - blurSize, 0.0);
    bokehAmount = pow(bokehAmount, 0.5);
    bokehAmount = min(bokehAmount, maxBlur) / maxBlur;
    vec2 direction = vec2(rand(uv), rand(uv));
    direction = normalize(direction);
    vec2 offset = direction * bokehAmount * bokehSize;
    vec4 bokeh = texture(inputImage, fract(uv + offset));
    color = mix(color, bokeh, bokehBrightness * bokehAmount);
    gl_FragColor = color;
}