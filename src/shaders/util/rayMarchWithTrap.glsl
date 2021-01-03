float distFromNearest(in vec3 p, out vec3 trap);

float rayMarchWithTrap(in vec3 ro, in vec3 rd, in float minHitDist,
                       in float maxDist, in int numSteps, out vec3 trap) {
    float totalDistancetraveled = 0.0;

    for (int i = 0; i < numSteps; i++) {
        vec3 currentPosition = ro + totalDistancetraveled * rd;
        float dist = distFromNearest(currentPosition, trap);

        if (totalDistancetraveled > maxDist) {
            break;
        }

        if (dist < minHitDist) {
            return totalDistancetraveled;
        }

        totalDistancetraveled += dist;
    }

    return -1.0;
}

float rayMarchWithTrap(in vec3 ro, in vec3 rd, in float accuracy,
                       out vec3 trap) {
    return rayMarchWithTrap(ro, rd, MIN_HIT_DISTANCE / accuracy,
                            MAX_TRACE_DISTANCE * accuracy,
                            NUM_STEPS / int(accuracy), trap);
}

float rayMarchWithTrap(in vec3 ro, in vec3 rd, out vec3 trap) {
    return rayMarchWithTrap(ro, rd, 1.0, trap);
}

float rayMarchWithTrapFast(in vec3 ro, in vec3 rd, out vec3 trap) {
    return rayMarchWithTrap(ro, rd, 0.5, trap);
}
