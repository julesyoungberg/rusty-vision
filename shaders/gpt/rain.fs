/*
{
    "DESCRIPTION": "Rain Generator",
    "CREDIT": "Created by ChatGPT",
    "ISFVSN": "2.0",
    "CATEGORIES": ["generator"],
    "INPUTS": [
        {
            "NAME": "rain_color",
            "TYPE": "color",
            "DEFAULT": [1.0, 1.0, 1.0, 1.0]
        },
        {
            "NAME": "rain_speed",
            "TYPE": "float",
            "DEFAULT": 0.3,
            "MIN": 0.0,
            "MAX": 2.0
        },
        {
            "NAME": "rain_width",
            "TYPE": "float",
            "DEFAULT": 2.0,
            "MIN": 0.0,
            "MAX": 10.0
        },
        {
            "NAME": "rain_density",
            "TYPE": "float",
            "DEFAULT": 0.1,
            "MIN": 0.0,
            "MAX": 1.0
        }
    ]
}
*/

vec2 hash2( vec2 p ) { return fract(sin(vec2(dot(p,vec2(127.1,311.7)),dot(p,vec2(269.5,183.3))))*43758.5453); }
float noise( in vec2 p ) {
    vec2 i = floor( p );
    vec2 f = fract( p );
    vec2 u = f*f*(3.0-2.0*f);
    return mix( mix( dot( hash2( i + vec2(0.0,0.0) ), f - vec2(0.0,0.0) ), 
                        dot( hash2( i + vec2(1.0,0.0) ), f - vec2(1.0,0.0) ), u.x),
                mix( dot( hash2( i + vec2(0.0,1.0) ), f - vec2(0.0,1.0) ), 
                        dot( hash2( i + vec2(1.0,1.0) ), f - vec2(1.0,1.0) ), u.x), u.y);
}
void main() {
    vec2 uv = gl_FragCoord.xy / RENDERSIZE.xy;
    vec2 pos = vec2(uv.x, uv.y + TIME * rain_speed);
    float noiseVal = noise(vec2(pos.x * rain_width, pos.y));
    float rainDrop = smoothstep(0.0, rain_density, noiseVal);
    vec4 rain = vec4(rain_color.rgb, rainDrop * rain_color.a);
    gl_FragColor = mix(vec4(0.0, 0.0, 0.0, 1.0), rain, rain.a);
}