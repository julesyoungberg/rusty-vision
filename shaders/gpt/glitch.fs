/*
{
    "DESCRIPTION" : "Glitch Effect",
    "CREDIT" : "Created by ChatGPT",
    "CATEGORIES" : ["filter"],
    "INPUTS" : [
        {
            "NAME": "inputImage",
            "TYPE": "image",
            "DESCRIPTION": "Input image"
        },
        {
            "NAME": "amplitude",
            "TYPE": "float",
            "DEFAULT": 0.05,
            "MIN": 0.0,
            "MAX": 1.0,
            "DESCRIPTION": "Amplitude of the glitch effect"
        },
        {
            "NAME": "speed",
            "TYPE": "float",
            "DEFAULT": 0.3,
            "MIN": 0.0,
            "MAX": 1.0,
            "DESCRIPTION": "Speed of the glitch effect"
        },
        {
            "NAME": "distortion",
            "TYPE": "float",
            "DEFAULT": 0.5,
            "MIN": 0.0,
            "MAX": 1.0,
            "DESCRIPTION": "Distortion factor"
        },
        {
            "NAME": "scanlines",
            "TYPE": "float",
            "DEFAULT": 0.1,
            "MIN": 0.0,
            "MAX": 1.0,
            "DESCRIPTION": "Scanline intensity"
        },
        {
            "NAME": "colorOffset",
            "TYPE": "float",
            "DEFAULT": 0.05,
            "MIN": 0.0,
            "MAX": 1.0,
            "DESCRIPTION": "Color offset"
        }
    ],
    "OUTPUTS" : [
        {
            "NAME" : "outputImage",
            "TYPE" : "image",
            "DESCRIPTION" : "Output image"
        }
    ]
}
*/

vec2 distort(vec2 uv, float amount) {
  vec2 offset = amount * vec2(
    sin(uv.y * 10.0 + TIME * speed),
    sin(uv.x * 10.0 + TIME * speed)
  );
  return uv + offset;
}

void main() {
  vec2 uv = gl_FragCoord.xy / RENDERSIZE.xy;
  vec2 distortedUv = distort(uv, distortion * amplitude);
  
  vec4 color = texture2D(inputImage, fract(distortedUv));
  vec4 scanlineColor = vec4(0.0, 0.0, 0.0, 1.0);
  if (mod(gl_FragCoord.y, 2.0) < scanlines) {
    color = mix(color, scanlineColor, colorOffset);
  }
  
  gl_FragColor = color;
}
