/*
{
  "ISFVSN": "2",
  "DESCRIPTION": "A psychedelic effect inspired by https://www.shadertoy.com/view/4sfGRn.",
  "CREDIT": "Inspired by https://www.shadertoy.com/view/4sfGRn",
  "CATEGORIES": [
    "filter"
  ],
  "INPUTS": [
    {
      "NAME": "inputImage",
      "DESCRIPTION": "The input image.",
      "TYPE": "image"
    }
  ]
}
*/

void main() {
    vec2 uv = isf_FragNormCoord.xy;
    vec4 color = texture(inputImage, uv);
    float t = TIME * 0.05;
    vec2 p = uv * 2.0 - 1.0;
    float r = length(p);
    float a = atan(p.y, p.x) + t;
    vec2 uv1 = vec2(cos(a), sin(a)) * r;
    vec2 uv2 = vec2(cos(a + sin(r + t) * 0.1), sin(a + cos(r + t) * 0.1)) * r;
    vec2 uv3 = vec2(cos(a + sin(r + t) * 0.2), sin(a + cos(r + t) * 0.2)) * r;
    vec4 color1 = texture(inputImage, uv1 * 0.5 + 0.5);
    vec4 color2 = texture(inputImage, uv2 * 0.5 + 0.5);
    vec4 color3 = texture(inputImage, uv3 * 0.5 + 0.5);
    color.rgb = vec3(color1.r, color2.g, color3.b);
    gl_FragColor = color;
}