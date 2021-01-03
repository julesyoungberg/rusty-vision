float marchRayWithTrap(const vec3 rayOrg, const vec3 rayDir,
                       const float startDist, out vec3 trap);
vec3 calculateNormal(vec3 position);
vec3 calculateColor(vec3 position, vec3 normal, vec3 eyePos, vec3 trap);

/**
 * To use the following must be defined
 * - RAY_PUSH
 * - REFLECTIVITY
 */
vec3 calculateReflectionsWithTrap(in vec3 position, in vec3 normal,
                                  in vec3 color, in vec3 eyePos, in vec3 bg) {
    vec3 rayDir = normalize(position - eyePos);
    vec3 reflectDir = reflect(rayDir, normal);

    vec3 orbitTrap;
    float dist = marchRayWithTrap(position, reflectDir, RAY_PUSH, orbitTrap);
    if (dist < 0.0) {
        return color;
    }

    vec3 surfacePos = position + reflectDir * dist;
    vec3 surfaceNorm = calculateNormal(surfacePos);
    vec3 finalColor =
        calculateColor(surfacePos, surfaceNorm, eyePos, orbitTrap);

    return mix(color, finalColor, REFLECTIVITY);
}
