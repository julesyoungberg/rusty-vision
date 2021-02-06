float distFromNearest(in vec3 pos);

/**
 * To use the following must be defined:
 * - RAY_PUSH
 * - MIN_HIT_DISTANCE
 * - SHADOW_FACTOR
 */
float calculateShadow(const vec3 position, const vec3 normal,
                      const vec3 lightPos) {
    vec3 rayDir = normalize(lightPos - position);
    float lightDist = length(lightPos - position);

    float accumulatedSoftShadows = 1.0;
    float minSoftShadows = 1.0;

    for (float totalDist = RAY_PUSH; totalDist < lightDist;) {
        vec3 currentPos = position + rayDir * totalDist;
        float currentDist = distFromNearest(currentPos);

        if (currentDist < MIN_HIT_DISTANCE) {
            accumulatedSoftShadows = 0.0;
            break;
        }

        float currentSoftShadows = SHADOW_FACTOR * currentDist / totalDist;
        totalDist += currentDist;
        minSoftShadows = min(minSoftShadows, currentSoftShadows);
    }

    accumulatedSoftShadows *= minSoftShadows;

    float shadowFactor = clamp(dot(rayDir, normal) * accumulatedSoftShadows,
                               1.0 - SHADOW_INTENSITY, 1.0);
    return shadowFactor;
}
