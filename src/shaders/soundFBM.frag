#version 450

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 frag_color;

layout(set = 0, binding = 0) uniform GeneralUniforms {
    vec2 resolution;
    float time;
};

layout(set = 1, binding = 0) uniform AudioUniforms {
    float dissonance;
    float energy;
    float loudness;
    float noisiness;
    float onset;
    float pitch;
    float rms;
    float spectralCentroid;
    float spectralComplexity;
    float spectralContrast;
    float tristimulus1;
    float tristimulus2;
    float tristimulus3;
};

#define NOISE_MODE 0
#define OCTAVES 4
#define INVERT false
#define SHARPEN true
#define SCALE_BY_PREV false

// Some useful functions
vec3 mod289(vec3 x) { return x - floor(x * (1.0 / 289.0)) * 289.0; }
vec2 mod289(vec2 x) { return x - floor(x * (1.0 / 289.0)) * 289.0; }
vec3 permute(vec3 x) { return mod289(((x*34.0)+1.0)*x); }
vec3 hsv2rgb(vec3 c) {
  vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
  vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
  return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}

//      Author : Ian McEwan, Ashima Arts
float snoise(vec2 v) {
    // Precompute values for skewed triangular grid
    const vec4 C = vec4(0.211324865405187, 
                        // (3.0-sqrt(3.0))/6.0
                        0.366025403784439,
                        // 0.5*(sqrt(3.0)-1.0)
                        -0.577350269189626,
                        // -1.0 + 2.0 * C.x
                        0.024390243902439);
                        // 1.0 / 41.0

    // First corner (x0)
    vec2 i  = floor(v + dot(v, C.yy));
    vec2 x0 = v - i + dot(i, C.xx);

    // Other two corners (x1, x2)
    vec2 i1 = vec2(0.0);
    i1 = (x0.x > x0.y)? vec2(1.0, 0.0):vec2(0.0, 1.0);
    vec2 x1 = x0.xy + C.xx - i1;
    vec2 x2 = x0.xy + C.zz;

    // Do some permutations to avoid
    // truncation effects in permutation
    i = mod289(i);
    vec3 p = permute(
            permute( i.y + vec3(0.0, i1.y, 1.0))
                + i.x + vec3(0.0, i1.x, 1.0 ));

    vec3 m = max(0.5 - vec3(
                        dot(x0,x0),
                        dot(x1,x1),
                        dot(x2,x2)
                        ), 0.0);

    m = m*m;
    m = m*m;

    // Gradients:
    //  41 pts uniformly over a line, mapped onto a diamond
    //  The ring size 17*17 = 289 is close to a multiple
    //      of 41 (41*7 = 287)

    vec3 x = 2.0 * fract(p * C.www) - 1.0;
    vec3 h = abs(x) - 0.5;
    vec3 ox = floor(x + 0.5);
    vec3 a0 = x - ox;

    // Normalise gradients implicitly by scaling m
    // Approximation of: m *= inversesqrt(a0*a0 + h*h);
    m *= 1.79284291400159 - 0.85373472095314 * (a0*a0+h*h);

    // Compute final noise value at P
    vec3 g = vec3(0.0);
    g.x  = a0.x  * x0.x  + h.x  * x0.y;
    g.yz = a0.yz * vec2(x1.x,x2.x) + h.yz * vec2(x1.y,x2.y);
    return 130.0 * dot(m, g);
}

float getNoiseVal(vec2 p) {
    float raw = snoise(p);

    if (NOISE_MODE == 1) {
        return abs(raw);
    }

    return raw * 0.5 + 0.5;
}

float fbm(vec2 p) {
    const float lacunarity = 2.0;
    const float gain = 0.5;
    float sum = 0.0;
    float freq = 1.0;
    float amp = 0.5;
    float prev = 1.0;

    for (int i = 0; i < OCTAVES; i++) {
        float n = getNoiseVal(p * freq);

        if (INVERT) {
            n = 1.0 - n;
        }

        if (SHARPEN) {
            n = n * n;
        }

        sum += n * amp;

        if (SCALE_BY_PREV) {
            sum += n * amp * prev;
        }

        prev = n;
        freq *= lacunarity;
        amp *= gain;
    }

    return sum;
}

float pattern(in vec2 p, out vec2 q, out vec2 r) {
    const float scale1 = 2.0;
    const float scale2 = 2.0;
    const float offset = 0.0;
    const vec2 offsetA = vec2(0.0);
    const vec2 offsetB = vec2(0.0);
    const vec2 offsetC = vec2(0.0);
    const vec2 offsetD = vec2(0.0);
    const vec2 timeScale = vec2(0.4, 0.3);  

    p *= scale1;
    p += offset;

    float t = time * 0.1;

    q = vec2(fbm(p + offsetA + t * timeScale.x), fbm(p + offsetB - t * timeScale.y));
    r = vec2(fbm(p + scale2 * q + offsetC), fbm(p + scale2 * q + offsetD));

    return fbm(p + scale2 * r);
}

void main() {
    vec3 finalColor = vec3(0.0);

    const vec3 color1 = hsv2rgb(vec3(tristimulus1, 1, 1));
    const vec3 color2 = hsv2rgb(vec3(tristimulus2, 1, 1));
    const vec3 color3 = hsv2rgb(vec3(tristimulus3, 1, 1));
    const vec3 color4 = vec3(0);

    vec2 q;
    vec2 r;
    float f = pattern(uv, q, r);
    
    finalColor = mix(color1, color2, f);
    finalColor = mix(finalColor, color3, length(q) / 2.0);
    finalColor = mix(finalColor, color4, r.y / 2.0);

    frag_color = vec4(finalColor,1.0);
}
