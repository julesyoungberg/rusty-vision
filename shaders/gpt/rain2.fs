/*{
  "DESCRIPTION": "Procedural Rain Generator",
  "CREDIT": "Created by [Your Name]"
}*/

/*{
  "INPUTS": [
    {
      "NAME": "speed",
      "TYPE": "float",
      "DEFAULT": 1.0,
      "MIN": 0.1,
      "MAX": 10.0
    },
    {
      "NAME": "density",
      "TYPE": "float",
      "DEFAULT": 0.5,
      "MIN": 0.1,
      "MAX": 1.0
    },
    {
      "NAME": "thickness",
      "TYPE": "float",
      "DEFAULT": 1.0,
      "MIN": 0.1,
      "MAX": 10.0
    },
    {
      "NAME": "brightness",
      "TYPE": "float",
      "DEFAULT": 1.0,
      "MIN": 0.1,
      "MAX": 1.0
    },
    {
      "NAME": "rain_color",
      "TYPE": "color",
      "DEFAULT": [0.5, 0.5, 0.5, 1.0]
    }
  ]
}*/

void main() {
  vec2 uv = isf_FragNormCoord.xy;
  float t = TIME * speed;

  // Generate the raindrop pattern
  float y = mod(uv.y + t * 0.2, 1.0);
  float x = mod(uv.x + y * 0.1, 1.0);
  float r = smoothstep(0.0, 0.003 * thickness, fract(x * 500.0));

  // Create the final rain effect with color
  float density = density * 2.0;
  vec3 color = mix(vec3(1.0), rain_color.rgb, r);
  vec3 rain = mix(vec3(0.0), color, smoothstep(1.0 - density, 1.0, y)) * brightness;

  vec4 outputColor = vec4(rain, 1.0);
  gl_FragColor = outputColor;
}
