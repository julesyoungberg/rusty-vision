/* 
{
  "DESCRIPTION": "Adds a magical sparkle effect to a video",
  "CREDIT": "Created by OpenAI GPT-3 and formatted by [Your Name Here]",
  "ISFVSN": "2.0",
  "CATEGORIES": [
    "generator",
    "filter"
  ],
  "INPUTS": [
    {
      "NAME": "inputImage",
      "TYPE": "image"
    },
    {
      "NAME": "speed",
      "TYPE": "float",
      "DEFAULT": 1.0,
      "MIN": 0.0,
      "MAX": 5.0
    },
    {
      "NAME": "sparkleSize",
      "TYPE": "float",
      "DEFAULT": 0.07,
      "MIN": 0.0,
      "MAX": 1.0
    },
    {
      "NAME": "sparkleIntensity",
      "TYPE": "float",
      "DEFAULT": 0.6,
      "MIN": 0.0,
      "MAX": 1.0
    }
  ]
}
*/

void main() {
  vec2 uv = isf_FragNormCoord.xy;
  vec4 col = texture(inputImage, uv);

  float time = TIME * speed;

  vec3 sparkleColor = vec3(1.0, 1.0, 1.0);
  vec4 sparkle = vec4(0.0);

  for (float i = 0.0; i < 5.0; i += 1.0) {
    vec2 sparklePos = vec2(fract(uv.x + i * 0.2), fract(uv.y + i * 0.2));
    vec2 sparkleDist = abs(sparklePos - vec2(0.5));
    float sprkl = 1.0 - smoothstep(sparkleSize - 0.01, sparkleSize + 0.01, length(sparkleDist * 2.0));
    sprkl *= sparkleIntensity;
    sprkl *= 1.0 - col.a;
    sprkl *= 1.0 - length(sparkleDist * 2.0);
    sprkl *= 1.0 - pow(abs(sin(time * 10.0)), 20.0);
    sparkleColor = vec3(1.0, 1.0, 1.0) * sprkl;
    sparkle += vec4(sparkleColor, 1.0);
  }

  col += sparkle;
  col.rgb = mix(col.rgb, sparkle.rgb, sparkleIntensity);

  gl_FragColor = col;
}