/*{
    "DESCRIPTION": "Audio reactive glitch effects",
    "CREDIT": "by julesyoungberg",
    "ISFVSN": "2.0",
    "CATEGORIES": [ "Glitch" ],
    "INPUTS": [
        {
            "NAME": "inputImage",
            "TYPE": "image"
        },
        {
            "NAME": "variance_threshold",
            "TYPE": "float",
            "MIN": 0.01,
            "MAX": 1.0,
            "DEFAULT": 0.1
        },
        {
            "NAME": "color_mix",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 1.0,
            "DEFAULT": 0.0
        },
        {
            "NAME": "edge_mode",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 3.0,
            "DEFAULT": 1.0
        },
        {
            "NAME": "min_divisions",
            "TYPE": "float",
            "MIN": 1.0,
            "MAX": 8.0,
            "DEFAULT": 2.0
        },
        {
            "NAME": "max_iterations",
            "TYPE": "float",
            "MIN": 1.0,
            "MAX": 10.0,
            "DEFAULT": 6.0
        }
    ]
}*/

// based on
// https://ciphrd.com/2020/04/02/building-a-quadtree-filter-in-glsl-using-a-probabilistic-approach/

#define SAMPLES_PER_ITERATION 30

vec3 image_color(in vec2 coord) {
    return IMG_NORM_PIXEL(inputImage, fract(coord)).rgb;
}

vec2 hash22(vec2 p) {
    float n = sin(dot(p, vec2(41, 289)));
    return fract(vec2(262144, 32768) * n);
}

vec4 color_variation(in vec2 center, in float size) {
    vec3 samples[SAMPLES_PER_ITERATION];

    vec3 color_mean = vec3(0.0);

    for (int i = 0; i < SAMPLES_PER_ITERATION; i++) {
        vec2 rnd = hash22(center.xy + vec2(float(i), 0.0)) - 0.5;
        vec2 sample_coord = center + rnd * size;
        samples[i] = image_color(sample_coord);
        color_mean += samples[i];
    }

    color_mean /= float(SAMPLES_PER_ITERATION);

    vec3 color_variance = vec3(0.0);

    // compute variance
    for (int i = 0; i < SAMPLES_PER_ITERATION; i++) {
        color_variance += pow(samples[i], vec3(2.0));
    }

    color_variance /= float(SAMPLES_PER_ITERATION);
    color_variance -= pow(color_mean, vec3(2.0));

    float variance =
        (color_variance.r + color_variance.g + color_variance.b) / 3.0;

    return vec4(color_mean, variance);
}

void main() {
    vec2 st = isf_FragNormCoord;
    vec3 color = vec3(0.0);
    vec2 center = vec2(0.5);
    vec4 variation = vec4(0.0);
    float divisions = 1.0;

    int min_iterations = int(floor(min_divisions));
    int end = min_iterations + int(floor(max_iterations));

    // loop until the variance is under the threshold
    for (int i = min_iterations; i < end; i++) {
        divisions = pow(2.0, float(i));
        center = (floor(st * divisions) + 0.5) / divisions;
        float side_length = 1 / divisions;
        variation = color_variation(center, side_length);

        if (variation.a < variance_threshold) {
            break;
        }
    }

    color = mix(image_color(st), variation.rgb, color_mix);

    // compute distance to edges
    vec2 uv = fract(st * divisions);
    vec2 l_width = 1.0 / RENDERSIZE;
    vec2 uv_abs = abs(uv - 0.5);
    float s = step(0.5 - uv_abs.x, l_width.x * divisions) +
              step(0.5 - uv_abs.y, l_width.y * divisions);

    // subtract edges
    if (edge_mode < 1.0) {
        color -= s;
    } else if (edge_mode < 2.0) {
        // invert edge pixels
        color = mix(color, 1.0 - color, s);
    } else {
        // lines only
        color = mix(vec3(0.0), 1.0 - color, s);
    }

    gl_FragColor = vec4(color, 1.0);
}
