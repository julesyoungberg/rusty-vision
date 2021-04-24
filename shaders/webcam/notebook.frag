#version 450

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 frag_color;

layout(set = 0, binding = 0) uniform GeneralUniforms {
    vec2 mouse;
    vec2 resolution;
    float time;
    int mouse_down;
};

layout(set = 1, binding = 0) uniform sampler webcam_sampler;
layout(set = 1, binding = 1) uniform texture2D webcam;
layout(set = 1, binding = 2) uniform WebcamUniforms {
    vec2 video_size;
};

#define TAU 6.28318530717959

//@import util/rand

vec2 rand2(vec2 p);
float rand21(vec2 p);

vec3 webcam_color(in vec2 coord) {
    vec2 c = vec2(coord.x, 1.0 - coord.y);
    return texture(sampler2D(webcam, webcam_sampler), fract(c)).rgb;
}

// a disection of notebook drawings by flockaroo
// https://www.shadertoy.com/view/XtVGD1
vec3 get_color(vec2 pos) {
    // take aspect ratio into account and fetch color
    vec2 uv = ((pos - resolution * 0.5) / resolution.y * video_size.y) / video_size.xy + 0.5;
    vec3 c1 = webcam_color(uv);
    
    // vec4 e = smoothstep(vec4(-0.05), vec4(-0.0), vec4(uv, vec2(1) - uv));
    // c1 = mix(vec4(1, 1, 1, 0), c1, e.x * e.y * e.z * e.w);

    // smooth, darken and brighten extremes to be more even
    float d = clamp(dot(c1, vec3(-0.5, 1.0, -0.5)), 0.0, 1.0);
    vec3 c2 = vec3(0.7);
    return min(mix(c1, c2, 1.8 * d), 0.7);
}

// get color and apply randomness
vec3 get_color_ht(vec2 pos) {
 	return smoothstep(0.95, 1.05, get_color(pos) * 0.8 + 0.2 + vec3(rand2(pos * 0.7), rand21(pos * 1.3)));
}

// get greyscale pixel
float get_greyscale(vec2 pos) {
    vec3 c = get_color(pos);
 	return pow(dot(c, vec3(0.333)), 1.0) * 1.0;
}

// get gradient of image
vec2 get_grad(vec2 pos, float eps) {
   	vec2 d = vec2(eps, 0);
    return vec2(
        get_greyscale(pos + d.xy) - get_greyscale(pos - d.xy),
        get_greyscale(pos + d.yx) - get_greyscale(pos - d.yx)
    ) / (eps * 2.0);
}

void main() {
    vec2 st = uv * 0.5 + 0.5;
    float r = resolution.y / 400.0;
    vec2 pos = st * resolution + 4.0 * sin(time * vec2(1.0, 1.7)) * r;
    
    vec3 color1 = vec3(0.0);
    vec3 color2 = vec3(0.0);

    const float angle_num = 3.0;
    const float sample_num = 4.0;

    float sum = 0.0;

    // loops around in a circle
    for (float i = 0.0; i < angle_num; i++) {
        float angle = TAU / angle_num * (i + 8.0);
        vec2 v = vec2(cos(angle), sin(angle));

        // loop through anti aliasing samples
        for (float j = 0.0; j < sample_num; j++) {
            // create two shifts based on the angle
            vec2 dpos = v.yx * vec2(1.0, -1.0) * j * r;
            vec2 dpos2 = v.xy * j * j / sample_num * 0.5 * r;
            vec2 g;
            float fact;
            float fact2;

            // convolve input image to get lines and smoothed colors
            for (float s = -1.0; s <= 1.0; s++) {
                vec2 pos2 = pos + s * dpos + dpos2;
                vec2 pos3 = pos + (s * dpos + dpos2).yx * vec2(1.0, -1.0) * 2.0;

                g = get_grad(pos2, 0.4);
                fact = dot(g, v) - 0.5 * abs(dot(g, v.yx * vec2(1.0, -1.0)));
                fact2 = dot(normalize(g + vec2(0.0001)), v.yx * vec2(1.0, -1.0));

                fact = clamp(fact, 0.0, 0.05);
                fact2 = abs(fact2);

                fact *= 1.0 - j / sample_num;
                color1 += fact;
                color2 += fact2 * get_color_ht(pos3);
                sum += fact2;
            }
        }
    }

    // normalize summations
    color1 /= (sample_num * angle_num) * 0.75 / sqrt(resolution.y);
    color2 /= sum;
    
    // invert r channel of color 1 to get dark edges
    color1.r *= (0.6 + 0.8 * rand21(pos * 0.7));
    color1.x = 1.0 - color1.x;
    color1.x *= color1.x * color1.x;

    // multiply color2 by the new dark edges
    color1 = color1.x * color2;

	frag_color = vec4(color1, 1.0);
}
