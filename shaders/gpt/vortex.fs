/*
{
    "DESCRIPTION" : "Vortex Effect",
    "CREDIT" : "Created by ChatGPT",
    "CATEGORIES" : ["filter"],
    "INPUTS" : [
        {
            "NAME": "inputImage",
            "TYPE": "image",
            "DESCRIPTION": "Input image"
        },
        {
            "NAME": "intensity",
            "TYPE": "float",
            "DEFAULT": 1.0,
            "MIN": 0.0,
            "MAX": 5.0,
            "DESCRIPTION": "Intensity of the effect"
        },
        {
            "NAME": "radius",
            "TYPE": "float",
            "DEFAULT": 0.5,
            "MIN": 0.0,
            "MAX": 1.0,
            "DESCRIPTION": "Radius of the vortex"
        },
        {
            "NAME": "angle",
            "TYPE": "float",
            "DEFAULT": 0.0,
            "MIN": 0.0,
            "MAX": 360.0,
            "DESCRIPTION": "Angle of the vortex"
        },
        {
            "NAME": "center_x",
            "TYPE": "float",
            "DEFAULT": 0.5,
            "DESCRIPTION": "X Center point of the vortex"
        },
        {
            "NAME": "center_y",
            "TYPE": "float",
            "DEFAULT": 0.5,
            "DESCRIPTION": "Y Center point of the vortex"
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

void main()
{
    vec2 uv = gl_FragCoord.xy / RENDERSIZE.xy;
    vec2 p = uv;
    vec2 center = vec2(center_x, center_y);
    vec2 q = p - center;
    float dist = length(q);
    float angle = atan(q.y, q.x);
    float sinAngle = sin(angle + angle * intensity);
    float cosAngle = cos(angle + angle * intensity);
    float newRadius = dist * radius;
    vec2 newPos = vec2(newRadius * cosAngle, newRadius * sinAngle) + center;
    vec4 color = texture2D(inputImage, vec2(newPos));
    gl_FragColor = color;
}
