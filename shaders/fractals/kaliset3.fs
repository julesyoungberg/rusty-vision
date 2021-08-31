/*{
    "DESCRIPTION": "",
    "CREDIT": "by julesyoungberg",
    "ISFVSN": "2.0",
    "CATEGORIES": [ "GENERATOR" ],
    "INPUTS": [
        {
            "NAME": "iterations",
            "TYPE": "float",
            "MIN": 1,
            "MAX": 50,
            "DEFAULT": 40
        },
        {
            "NAME": "c_x",
            "TYPE": "float",
            "MIN": -1.0,
            "MAX": 1.0,
            "DEFAULT": -0.32
        },
        {
            "NAME": "c_y",
            "TYPE": "float",
            "MIN": -1.0,
            "MAX": 1.0,
            "DEFAULT": 0.87
        },
        {
            "NAME": "speed",
            "TYPE": "float",
            "MIN": -1.0,
            "MAX": 1.0,
            "DEFAULT": 0.09
        },
        {
            "NAME": "color_config",
            "TYPE": "color",
            "DEFAULT": [
                0.9,
                1.0,
                1.0,
                1.0
            ]
        },
        {
            "NAME": "color_offset",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 1.0,
            "DEFAULT": 0.0
        },
        {
            "NAME": "color_scale",
            "TYPE": "float",
            "MIN": 0.01,
            "MAX": 1.0,
            "DEFAULT": 0.1
        }
    ]
}*/

const vec3 LIGHT_POS = vec3(0.0, 0.0, -1.0);

vec2 complex_inv(in vec2 z) {
    vec2 conjugate = vec2(z.x, -z.y);
    float denominator = dot(conjugate, conjugate);
    return conjugate / denominator;
}

vec2 complex_log(in vec2 z) { return vec2(log(length(z)), atan(z.y, z.x)); }

vec2 complex_mult(in vec2 a, in vec2 b) {
    return vec2(a.x * b.x - a.y * b.y, a.x * b.y + a.y * b.x);
}

// IQ's palette generator:
// https://www.iquilezles.org/www/articles/palettes/palettes.htm
vec3 palette(in float t, in vec3 a, in vec3 b, in vec3 c, in vec3 d) {
    return a + b * cos(6.28318 * (c * t + d));
}

// Based on Alien Tech by Kali
// https://www.shadertoy.com/view/XtX3zj
// formulas from
// http://www.fractalforums.com/new-theories-and-research/very-simple-formula-for-fractal-patterns
vec2 formula(in vec2 st) {
    vec2 z = st;

    vec2 c = vec2(-0.6);
    c = vec2(c_x + sin(TIME * speed) * 0.05, c_y);

    float expsmo = 0.0;
    float len = 0.0;
    float orbit_trap = 0.0;

    float angle = TIME * speed;

    for (float i = 0.0; i < iterations; i++) {
        // rotation
        // z *= mat2(cos(angle), -sin(angle), sin(angle), cos(angle));

        // original kali equation
        // z = abs(z) / dot(z, z) + c;

        // kali variations
        // z = abs(z) / (z.x * z.y) + c;
        // z = abs(complex_inv(z)) + c;
        // z = complex_inv(abs(z)) + c;
        // z = complex_log(abs(complex_inv(z)) + c);
        // z = complex_mult(abs(z), complex_inv(abs(c))) + c;
        // z = complex_mult(abs(z), complex_inv(c)) + c;
        // z = abs(complex_mult(z, complex_inv(c))) + c;
        // z = abs(complex_mult(z, z)) + c;
        // z = abs(complex_inv(complex_mult(complex_mult(z, z), z))) + c;
        // z = complex_inv(complex_mult(complex_mult(abs(z), abs(z)), abs(z))) +
        // c;

        // softology variations
        z.x = -abs(z.x);
        const vec2 cone = vec2(1.0, 0.0);
        // z = complex_mult(z, c) + cone + complex_inv(complex_mult(z, c) +
        // cone); z = abs(complex_mult(z, c) + cone) +
        // complex_inv(abs(complex_mult(z, c) + cone));
        vec2 temp = abs(complex_mult(z, c) + cone);
        z = temp + complex_mult(cone, complex_inv(temp));

        float mag = length(z);

        // exponential smoothing
        if (mod(i, 2.0) < 1.0) {
            float prev_len = len;
            len = mag;
            expsmo += exp(-1.0 / abs(len - prev_len));
            orbit_trap = min(orbit_trap, len);
        }
    }

    return vec2(expsmo, orbit_trap);
}

vec3 light(vec2 p, vec3 color) {
    // calculate normals based on horizontal and vertical vectors being z the
    // formula result
    const vec2 d = vec2(0.0, 0.01);
    float d1 = formula(p - d.xy).x - formula(p + d.xy).x;
    float d2 = formula(p - d.yx).x - formula(p + d.yx).x;
    vec3 n1 = vec3(0.0, d.y * 2.0, -d1 * 0.05);
    vec3 n2 = vec3(d.y * 2.0, 0.0, -d2 * 0.05);
    vec3 n = normalize(cross(n1, n2));

    // lighting
    vec3 light_dir = normalize(vec3(p, 0.0) + LIGHT_POS);
    float diff = pow(max(0.0, dot(light_dir, n)), 2.0) +
                 0.2; // lambertian diffuse + ambient
    vec3 r = reflect(vec3(0.0, 0.0, 1.0), light_dir); // half vector
    float spec = pow(max(0.0, dot(r, n)), 30.0);      // specular
    return diff * color + spec * 0.1;
}

void main() {
    vec2 st = isf_FragNormCoord * 2.0 - 1.0;
    float aspect_ratio = RENDERSIZE.x / RENDERSIZE.y;
    st.x *= aspect_ratio;
    st *= 1.0;

    vec3 color = vec3(0);

    float t = TIME;
    // float scale = 1.0 + 0.5 * sin(t / 17.0);
    // st *= scale;

    vec2 pix_size = 0.25 / RENDERSIZE;
    pix_size.x *= aspect_ratio;
    const float aa_samples = 1.0;
    const float aa_sqrt = sqrt(aa_samples);
    float little_lights = 0.0;

    vec2 m = 2.0 * sin(vec2(TIME)) / RENDERSIZE.y;
    m = m * 0.5 + 0.5;

    for (float aa = 0.0; aa < aa_samples; aa++) {
        vec2 aa_coord = floor(vec2(aa / aa_sqrt, mod(aa, aa_sqrt)));
        vec2 p = st + aa_coord * pix_size;

        vec2 result = formula(p);
        float k = clamp(result.x * 0.06, 0.8, 1.4);
        vec3 col =
            palette(result.x * color_scale + color_offset, vec3(0.5, 0.5, 0.5),
                    vec3(0.5, 0.5, 0.5), color_config.rgb, vec3(m, 1.0));
        col *= 0.4;
        // col = 1.0 - col;

        color += light(p, col);
        little_lights += max(0.0, 2.0 - result.y) / 2.0;
    }

    color /= aa_samples;

    // uv shift by light coords
    vec2 luv = st + LIGHT_POS.xy;

    // min amb light + spotlight with falloff * varying intensity
    color *= 0.07 + pow(max(0.0, 2.0 - length(luv) * 0.5), 2.0);

    // yellow lights
    // color += pow(little_lights * 0.12, 15.0) * vec3(1.0, 0.9, 0.3) * (0.8 +
    // sin(TIME * 5.0 - st.y * 10.0) * 0.6);

    gl_FragColor = vec4(color, 1);
}
