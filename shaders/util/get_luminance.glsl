// https://github.com/CesiumGS/cesium/blob/master/Source/Shaders/Builtin/Functions/luminance.glsl
float get_luminance(vec3 rgb) {
    // Algorithm from Chapter 10 of Graphics Shaders.
    const vec3 W = vec3(0.2125, 0.7154, 0.0721);
    return dot(rgb, W);
}
