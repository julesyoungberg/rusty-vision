float distFromNearest(in vec3 p);

float rayMarch(in vec3 ro, in vec3 rd, in float minHitDist, in float maxDist,
               in int numSteps, in bool internal) {
    float totalDistancetraveled = 0.0;

    for (int i = 0; i < numSteps; i++) {
        vec3 currentPosition = ro + totalDistancetraveled * rd;
        float dist = distFromNearest(currentPosition);
        if (internal) {
            dist *= -1.0;
        }

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

float rayMarch(in vec3 ro, in vec3 rd, in float accuracy, in bool internal) {
    return rayMarch(ro, rd, MIN_HIT_DISTANCE / accuracy,
                    MAX_TRACE_DISTANCE * accuracy, NUM_STEPS / int(accuracy),
                    internal);
}

float rayMarch(in vec3 ro, in vec3 rd, in float accuracy) {
    return rayMarch(ro, rd, accuracy, false);
}

float rayMarch(in vec3 ro, in vec3 rd) { return rayMarch(ro, rd, 1.0); }

float rayMarchFast(in vec3 ro, in vec3 rd) { return rayMarch(ro, rd, 0.5); }

float rayMarchInternal(in vec3 ro, in vec3 rd, in float accuracy) {
    return rayMarch(ro, rd, accuracy, true);
}

float rayMarchInternal(in vec3 ro, in vec3 rd) {
    return rayMarchInternal(ro, rd, 1.0);
}

float rayMarchInternalFast(in vec3 ro, in vec3 rd) {
    return rayMarchInternal(ro, rd, 0.5);
}
