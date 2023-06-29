/*
{
  "DESCRIPTION": "Generates a trippy, organic pattern using Perlin noise and color gradients.",
  "CREDIT": "ChatGPT",
  "ISFVSN": "2.0",
  "CATEGORIES": [
    "generator",
    "artistic"
  ],
  "INPUTS": [
    {
      "NAME": "scale",
      "TYPE": "float",
      "DEFAULT": 2.0,
      "MIN": 0.1,
      "MAX": 10.0,
      "CLAMP": true,
      "PRECISION": 1
    },
    {
      "NAME": "frequency",
      "TYPE": "float",
      "DEFAULT": 2.0,
      "MIN": 0.1,
      "MAX": 10.0,
      "CLAMP": true,
      "PRECISION": 1
    },
    {
      "NAME": "speed",
      "TYPE": "float",
      "DEFAULT": 1.0,
      "MIN": 0.1,
      "MAX": 5.0,
      "CLAMP": true,
      "PRECISION": 1
    }
  ]
}
*/

void main() {
  vec2 fragCoord = isf_FragNormCoord;
  // Set up the scales and frequencies
  float noiseScale = 2.0 * scale;
  float xFreq = frequency;
  float yFreq = frequency * 1.2;

  // Calculate the noise values for the x and y coordinates
  float noiseX = (1.0 + sin(fragCoord.x * xFreq + TIME * speed)) * 0.5 * 2.0 - 1.0;
  float noiseY = (1.0 + sin(fragCoord.y * yFreq + TIME * speed)) * 0.5 * 2.0 - 1.0;

  // Combine the noise values to generate the final pattern
  float pattern = abs(noiseX + noiseY) * 0.5 + 0.5;

  // Create the color gradient for the pattern
  vec4 color1 = vec4(1.0, 0.0, 0.0, 1.0);
  vec4 color2 = vec4(0.0, 1.0, 0.0, 1.0);
  vec4 color3 = vec4(0.0, 0.0, 1.0, 1.0);
  vec4 color4 = vec4(1.0, 1.0, 0.0, 1.0);
  vec4 color5 = vec4(1.0, 0.0, 1.0, 1.0);
  vec4 color6 = vec4(0.0, 1.0, 1.0, 1.0);
  vec4 color7 = vec4(1.0, 1.0, 1.0, 1.0);
  gl_FragColor = mix(mix(mix(color1, color2, pattern), mix(color3, color4, pattern), pattern),
                  mix(mix(color5, color6, pattern), color7, pattern), pattern);
}
