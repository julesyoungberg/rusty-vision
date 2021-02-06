float distFromNearest(in vec3 p);

float calculateSoftShadow(vec3 position, vec3 lightPos, float k) {
    const int maxIterationsShad = 24;
    vec3 rayDir = lightPos - position;

    float shade = 1.0;
    float dist = .002;
    float end = max(length(rayDir), .001);
    float stepDist = end / float(maxIterationsShad);
    rayDir /= end;

    rayDir = normalize(rayDir);

    for (int i = 0; i < maxIterationsShad; i++) {
        float h = distFromNearest(position + rayDir * dist);
        shade = min(shade, smoothstep(0., 1., k * h / dist));
        dist += clamp(h, .02, .25);
        if (h < 0.0 || dist > end) {
            break;
        }
    }

    return min(max(shade, 0.0) + 0.25, 1.0);
}
