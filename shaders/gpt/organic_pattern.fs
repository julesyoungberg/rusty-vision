
/*
{
    "INPUTS": [
        {
            "NAME": "scale",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 1.0,
            "DEFAULT": 0.5
        },
        {
            "NAME": "feedback",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 1.0,
            "DEFAULT": 0.5
        },
        {
            "NAME": "colorMode",
            "TYPE": "int",
            "MIN": 0,
            "MAX": 5,
            "DEFAULT": 0
        }
    ]
}
*/

vec3 mod289(vec3 x) {
    return x - floor(x * (1.0 / 289.0)) * 289.0;
}

vec2 mod289(vec2 x) {
    return x - floor(x * (1.0 / 289.0)) * 289.0;
}

vec3 permute(vec3 x) {
    return mod289(((x*34.0)+1.0)*x);
}

float snoise(vec2 v)
{
    const vec4 C = vec4(0.211324865405187,  // (3.0-sqrt(3.0))/6.0
                        0.366025403784439,  // 0.5*(sqrt(3.0)-1.0)
                        -0.577350269189626,  // -1.0 + 2.0 * C.x
                        0.024390243902439); // 1.0 / 41.0
    // First corner
    vec2 i  = floor(v + dot(v, C.yy) );
    vec2 x0 = v -   i + dot(i, C.xx);

    // Other corners
    vec2 i1;
    //i1.x = step( x0.y, x0.x ); // x0.x > x0.y ? 1.0 : 0.0
    //i1.y = 1.0 - i1.x;
    i1 = (x0.x > x0.y) ? vec2(1.0, 0.0) : vec2(0.0, 1.0);
    // x0 = x0 - 0.0 + 0.0 * C.xx ;
    // x1 = x0 - i1 + 1.0 * C.xx ;
    // x2 = x0 - 1.0 + 2.0 * C.xx ;
    vec4 x12 = x0.xyxy + C.xxzz;
    x12.xy -= i1;

    // Permutations
    i = mod289(i); // Avoid truncation effects in permutation
    vec3 p = permute( permute( i.y + vec3(0.0, i1.y, 1.0 ))
        + i.x + vec3(0.0, i1.x, 1.0 ));

    vec3 m = max(0.5 - vec3(dot(x0,x0), dot(x12.xy,x12.xy), dot(x12.zw,x12.zw)), 0.0);
    m = m*m ;
    m = m*m ;

    // Gradients: 41 points uniformly over a line, mapped onto a diamond.
    // The ring size 17*17 = 289 is close to a multiple of 41 (41*7 = 287)

    vec3 x = 2.0 * fract(p * C.www) - 1.0;
    vec3 h = abs(x) - 0.5;
    vec3 ox = floor(x + 0.5);
    vec3 a0 = x - ox;

    // Normalise gradients implicitly by scaling m
    // Approximation of: m *= inversesqrt( a0*a0 + h*h );
    m *= 1.79284291400159 - 0.85373472095314 * ( a0*a0 + h*h );

    // Compute final noise value at P
    vec3 g;
    g.x  = a0.x  * x0.x  + h.x  * x0.y;
    g.yz = a0.yz * x12.xz + h.yz * x12.yw;
    return 130.0 * dot(m, g);
}

vec4 organicPattern(vec2 uv) {
    vec2 st = uv * scale;
    vec3 color = vec3(0.0);
    float n = snoise(st);

    if (colorMode == 0) { // "hue"
        color = vec3(fract(n * 0.5 + feedback), 1.0, 1.0);
    } else if (colorMode == 1) { // "saturation"
        color = vec3(fract(n * 0.5 + feedback), fract(n * 0.5 + feedback), 1.0);
    } else if (colorMode == 2) { // "value"
        color = vec3(fract(n * 0.5 + feedback), fract(n * 0.5 + feedback), fract(n * 0.5 + feedback));
    } else if (colorMode == 3) { // "red"
        color = vec3(fract(n * 0.5 + feedback), 0.0, 0.0);
    } else if (colorMode == 4) { // "green"
        color = vec3(0.0, fract(n * 0.5 + feedback), 0.0);
    } else if (colorMode == 5) { // "blue"
        color = vec3(0.0, 0.0, fract(n * 0.5 + feedback));
    }
    return vec4(color, 1.0);
}

void main(void) {
    vec2 uv = gl_FragCoord.xy / RENDERSIZE.xy;
    gl_FragColor = organicPattern(uv);
}
