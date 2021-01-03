float distFromNearest(in vec3 p);

/**
 * Similar to rayMarch except it doesn't care about steps, only ray length
 * To use the following must be defined:
 * - MAX_RAY_LENGTH
 * - MIN_HIT_DISTANCE
 */
float marchRay(const vec3 rayOrg, const vec3 rayDir, const float startDist) {
    for (float totalDist = startDist; totalDist < MAX_RAY_LENGTH;) {
        vec3 currentPos = rayOrg + rayDir * totalDist;
        float currentDist = distFromNearest(currentPos);
        if (currentDist < MIN_HIT_DISTANCE) {
            return totalDist;
        }
        totalDist += currentDist;
    }
    return -1.0;
}
