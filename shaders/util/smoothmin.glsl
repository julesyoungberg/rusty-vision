float smoothmin(float a, float b, float k) {
    float h = clamp((b - a) / k + 0.5, 0.0, 1.0);
    return mix(a, b, h) - h * (1.0 - h) * k * 0.5;
}
