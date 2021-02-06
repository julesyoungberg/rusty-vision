float rayMarch(in vec3 ro, in vec3 rd);
vec3 calculateNormal(in vec3 p);
vec3 calculateColor(in vec3 position, in vec3 normal, in vec3 eyePos);

vec3 getSurfaceColor(in vec3 origin, in vec3 rayDir, in vec3 bg) {
    float dist = rayMarch(origin, rayDir);
    if (dist < 0.0) {
        return bg;
    }

    vec3 surfacePos = origin + rayDir * dist;
    vec3 normal = calculateNormal(surfacePos);
    return calculateColor(surfacePos, normal, origin);
}

vec3 getSurfaceColor(in vec3 origin, in vec3 rayDir) {
    return getSurfaceColor(origin, rayDir, vec3(0));
}
