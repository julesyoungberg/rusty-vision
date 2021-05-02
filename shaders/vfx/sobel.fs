/*{
    "DESCRIPTION": "Audio reaactive glitch effects",
    "CREDIT": "by julesyoungberg",
    "ISFVSN": "2.0",
    "CATEGORIES": [ "FX" ],
    "INPUTS": [
        {
            "NAME": "input_image",
            "TYPE": "image"
        }
    ]
}*/

vec3 image_color(in vec2 coord) {
    return IMG_NORM_PIXEL(input_image, fract(coord)).rgb;
}

// sobel filter https://en.wikipedia.org/wiki/Sobel_operator
void main() {
    vec2 st = isf_FragNormCoord;

    vec3 color = vec3(0.0);

    const float d = 0.004;
    vec3 stp = vec3(-d, 0, d);
    float c00 = length(image_color(st + stp.xx));
    float c01 = length(image_color(st + stp.xy));
    float c02 = length(image_color(st + stp.xz));
    float c10 = length(image_color(st + stp.yx));
    float c12 = length(image_color(st + stp.yz));
    float c20 = length(image_color(st + stp.zx));
    float c21 = length(image_color(st + stp.zy));
    float c22 = length(image_color(st + stp.zz));

    float gx = c00 + 2.0 * c01 + c02 - c20 - 2.0 * c21 - c22;
    float gy = c00 + 2.0 * c10 + c20 - c02 - 2.0 * c12 - c22;
    float g = sqrt(gx * gx + gy * gy);

    color = vec3(0);
    color += step(1.0, g);

    gl_FragColor = vec4(color, 1.0);
}
