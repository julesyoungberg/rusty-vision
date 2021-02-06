float distFromNearest(in vec3 p);

float getShadowMultiplier(in vec3 position, in vec3 lightPos,
                          in float shadowConst, in float b) {
    vec3 lightRay = lightPos - position;
    vec3 lightDir = normalize(lightRay);
    float maxLength = length(lightDir);

    float finalDist = 1000.0;
    vec3 rayPos = position;
    float totalDist = 0.1;
    rayPos += totalDist * lightDir;
    float res = 1.0;

    for (int i = 0; i < NUM_STEPS && totalDist < maxLength; i++) {
        if (finalDist < MIN_HIT_DISTANCE) {
            return b;
        }

        finalDist = distFromNearest(rayPos);
        totalDist += finalDist;
        res = min(res, shadowConst * finalDist / totalDist);
        rayPos += finalDist * lightDir;
    }

    return res;
}
