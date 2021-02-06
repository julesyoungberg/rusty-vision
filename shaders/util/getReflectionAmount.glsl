float getReflectionAmount(in float n1, in float n2, in vec3 normal,
                          in vec3 incident, in float reflectivity) {
    // Schlick aproximation
    float r0 = (n1 - n2) / (n1 + n2);
    r0 *= r0;
    float cosX = -dot(normal, incident);

    if (n1 > n2) {
        float n = n1 / n2;
        float sinT2 = n * n * (1.0 - cosX * cosX);
        // Total internal reflection
        if (sinT2 > 1.0) {
            return 1.0;
        }
        cosX = sqrt(1.0 - sinT2);
    }

    float x = 1.0 - cosX;
    float ret = r0 + (1.0 - r0) * x * x * x * x * x;

    // adjust reflect multiplier for object reflectivity
    ret = (reflectivity + (1.0 - reflectivity) * ret);
    return ret;
}
