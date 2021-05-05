/*{
    "DESCRIPTION": "Dot halftone effect.",
    "CREDIT": "by julesyoungberg",
    "ISFVSN": "2.0",
    "CATEGORIES": [ "Halftone Effect" ],
    "INPUTS": [
        {
            "NAME": "inputImage",
            "TYPE": "image"
        }
    ]
}*/

// https://github.com/CesiumGS/cesium/blob/master/Source/Shaders/Builtin/Functions/luminance.glsl
float get_luminance(vec3 rgb) {
    // Algorithm from Chapter 10 of Graphics Shaders.
    const vec3 W = vec3(0.2125, 0.7154, 0.0721);
    return dot(rgb, W);
}

vec3 image_color(in vec2 coord) {
    return IMG_NORM_PIXEL(inputImage, fract(coord)).rgb;
}

void main() {
    vec2 st = isf_FragNormCoord;
    vec3 color = vec3(0);

    // tile the space
    float scale = 150.0;
    vec2 p = st;
    p.y *= RENDERSIZE.y / RENDERSIZE.x;
    p *= scale;
    vec2 gv = fract(p) - 0.5;
    vec2 id = floor(p);

    // get corresponding pixel brightness
    vec2 coord = (id + 0.5) / scale;
    coord.y *= RENDERSIZE.x / RENDERSIZE.y;
    vec3 image_color = image_color(coord);
    float brightness = get_luminance(image_color);

    // reduce number of shades
    float n_shades = 5.0;
    float shade = floor(mod(brightness * n_shades, n_shades)) / n_shades;

    // draw circle in each grid cell
    float r = 0.5 * shade;
    float d = smoothstep(r, r * 0.95, length(gv));

    color += d * shade;

    gl_FragColor = vec4(color, 1.0);
}
