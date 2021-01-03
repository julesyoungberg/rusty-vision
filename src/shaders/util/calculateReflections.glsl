float marchRay(const vec3 rayOrg, const vec3 rayDir, const float startDist);
vec3 calculateNormal(vec3 position);
vec3 calculateColor(vec3 position, vec3 normal, vec3 eyePos);

/**
 * To use the following must be defined
 * - RAY_PUSH
 * - REFLECTIVITY
 */
vec3 calculateReflections(in vec3 position, in vec3 normal, in vec3 color,
                          in vec3 eyePos, in vec3 bg) {
    vec3 rayDir = normalize(position - eyePos);
    vec3 reflectDir = reflect(rayDir, normal);

    float dist = marchRay(position, reflectDir, RAY_PUSH);
    if (dist < 0.0) {
        return color;
    }

    vec3 surfacePos = position + reflectDir * dist;
    vec3 surfaceNorm = calculateNormal(surfacePos);
    vec3 finalColor = calculateColor(surfacePos, surfaceNorm, eyePos);

    return mix(color, finalColor, REFLECTIVITY);
}
