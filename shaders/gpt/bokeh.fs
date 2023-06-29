/* 
{
  "DESCRIPTION": "Adds a bokeh effect to a video",
  "CREDIT": "Created by [Your Name Here]",
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
      "NAME": "bokehSize",
      "TYPE": "float",
      "DEFAULT": 0.2,
      "MIN": 0.0,
      "MAX": 1.0
    },
    {
      "NAME": "bokehIntensity",
      "TYPE": "float",
      "DEFAULT": 0.5,
      "MIN": 0.0,
      "MAX": 1.0
    },
    {
      "NAME": "blurAmount",
      "TYPE": "float",
      "DEFAULT": 0.01,
      "MIN": 0.0,
      "MAX": 1.0
    }
  ],
  "OUTPUTS": [
    {
      "NAME": "outputImage",
      "TYPE": "image"
    }
  ]
}
*/

void main() {
  vec2 uv = isf_FragNormCoord.xy;
  vec4 col = texture(inputImage, uv);

  float time = TIME * speed;

  vec3 bokehColor = vec3(1.0, 1.0, 1.0);
  vec4 bokeh = vec4(0.0);

  for (int i = 0; i < 5; i++) {
    vec2 bokehPos = vec2(fract(uv.x + float(i) * 0.2), fract(uv.y + float(i) * 0.2));
    vec2 bokehDist = abs(bokehPos - vec2(0.5));
    float bkh = 1.0 - smoothstep(bokehSize - 0.01, bokehSize + 0.01, length(bokehDist * 2.0));
    bkh *= bokehIntensity;
    bkh *= 1.0 - col.a;
    bkh *= 1.0 - length(bokehDist * 2.0);
    bkh *= 1.0 - pow(abs(sin(time * 10.0)), 20.0);
    bokehColor = vec3(1.0, 1.0, 1.0) * bkh;
    bokeh += vec4(bokehColor, 1.0);
  }

  bokeh = texture(inputImage, uv);
  bokeh += blurAmount / 4.0 * texture(inputImage, uv + vec2(0.0, 1.0) / 300.0);
  bokeh += blurAmount / 4.0 * texture(inputImage, uv + vec2(0.0, -1.0) / 300.0);
  bokeh += blurAmount / 4.0 * texture(inputImage, uv + vec2(1.0, 0.0) / 300.0);
  bokeh += blurAmount / 4.0 * texture(inputImage, uv + vec2(-1.0, 0.0) / 300.0);
  col = mix(col, bokeh, bokehIntensity);

  gl_FragColor = col;
}