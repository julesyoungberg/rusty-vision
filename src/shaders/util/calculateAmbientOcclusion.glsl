float distFromNearest(in vec3 p);

#define AO_NUM_STEPS 5.
#define AO_FIRST_STEP_DIST 0.005
#define AO_STEP_SIZE 0.02
#define AO_WEIGHT_MODIFIER 0.95
#define AO_STRENGTH 3.
#define AO_INTENSITY 0.99

float calculateAmbientOcclusion(in vec3 position, in vec3 normal) {
    float result = 0.0;

    float weight = 1.0;
    for (float i = 0.0; i < AO_NUM_STEPS; i++) {
        float dist = AO_FIRST_STEP_DIST + AO_STEP_SIZE * i;
        vec3 currentPos = position + normal * dist;
        float aoDist = distFromNearest(currentPos);
        float val = (dist - aoDist) * weight;
        result += val;
        weight *= AO_WEIGHT_MODIFIER;
    }

    result *= AO_STRENGTH;
    result = clamp(1.0 - result, 1.0 - AO_INTENSITY, 1.0);
    return result;
}
