/*
 * Calculates the distance from a ray origin to a plane.
 * Useful for calculating infinite floors.
 */
float calculateFloorDist(const vec3 rayOrigin, const vec3 rayDir,
                         const float level) {
    vec3 normal = vec3(0, 1, 0);
    vec3 center = vec3(0, level, 0);

    float angle = dot(normal, rayDir);

    // if the ray is not parallel with the floor
    if (angle < -EPSILON) {
        vec3 floorRayOrigin = rayOrigin - center;
        float dist = -dot(floorRayOrigin, normal) / dot(rayDir, normal);
        return dist;
    }

    return -1.0;
}