/*
{
    "DESCRIPTION" : "Organic Wave Effect",
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
            "DEFAULT": 0.1,
            "MIN": 0.0,
            "MAX": 1.0,
            "DESCRIPTION": "Amplitude of the wave"
        },
        {
            "NAME": "wavelength",
            "TYPE": "float",
            "DEFAULT": 0.05,
            "MIN": 0.0,
            "MAX": 1.0,
            "DESCRIPTION": "Wavelength of the wave"
        },
        {
            "NAME": "speed",
            "TYPE": "float",
            "DEFAULT": 0.5,
            "MIN": 0.0,
            "MAX": 1.0,
            "DESCRIPTION": "Speed of the wave"
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

void main() {
    vec2 uv = gl_FragCoord.xy / RENDERSIZE.xy;
    vec4 color = texture2D(inputImage, uv);
    
    float t = TIME * speed;
    float offset = amplitude * sin((uv.y + t) / wavelength);
    vec2 offsetUV = vec2(uv.x + offset, uv.y);
    
    vec4 waveColor = texture2D(inputImage, fract(offsetUV));
    
    gl_FragColor = mix(color, waveColor, amplitude * 0.5);
}
