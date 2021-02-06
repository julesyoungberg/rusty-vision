vec3 castRay(in vec2 uv, in vec3 camPos, in vec3 lookAt, in float zoom);
vec3 getSurfaceColor(in vec3 origin, in vec3 rayDir, in vec3 bg);
vec2 getUV(in vec2 coord, in vec2 res);
vec2 hash(float p);

vec3 getSmoothSurfaceColor(in vec2 coord, in vec2 res, in vec3 camPos,
                           in vec3 lookAt, in float zoom, in vec3 bg,
                           in int gridRes) {
    vec2 currentUV = getUV(coord, resolution);
    vec3 rayDir = castRay(currentUV, camPos, lookAt, zoom);
    vec3 finalColor = bg;
    float d = float(gridRes);

    int numSubPixels = int(pow(d, 2.0));
    for (int i = 1; i <= numSubPixels; i++) {
        float x = mod(float(i) - 1.0, d);
        float y = mod(floor(float(i) / d), d);
        vec2 jitter = hash(float(i)) / d;
        jitter.x += x / d;
        jitter.y += y / d;

        currentUV = getUV(coord + jitter, res);
        rayDir = castRay(currentUV, camPos, lookAt, zoom);
        vec3 color = getSurfaceColor(camPos, rayDir, bg);

        finalColor = mix(finalColor, color, 1.0 / float(i));
    }

    return pow(finalColor, vec3(1.0 / 1.3));
}
