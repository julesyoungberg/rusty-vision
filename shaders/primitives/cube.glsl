// compute the near and far intersections of the cube (stored in the x and y
// components) using the slab method no intersection means vec.x > vec.y (really
// tNear > tFar)
vec2 cubeIntersect(vec3 rayOrigin, vec3 rayDir, vec3 boxMin, vec3 boxMax) {
    vec3 tMin = (boxMin - rayOrigin) / rayDir;
    vec3 tMax = (boxMax - rayOrigin) / rayDir;
    vec3 t1 = min(tMin, tMax);
    vec3 t2 = max(tMin, tMax);
    float tNear = max(max(t1.x, t1.y), t1.z);
    float tFar = min(min(t2.x, t2.y), t2.z);
    return vec2(tNear, tFar);
}

float sdBox(in vec3 p, in vec3 b);

vec3 cubeNormal(vec3 pos, vec3 boxSize) {
    vec3 boxCenter = boxSize / 2.0;
    vec3 p = pos;

    const vec3 stp = vec3(1e-5, 0, 0);

    float x = sdBox(p + stp.xyy, boxSize) - sdBox(p - stp.xyy, boxSize);
    float y = sdBox(p + stp.yxy, boxSize) - sdBox(p - stp.yxy, boxSize);
    float z = sdBox(p + stp.yyx, boxSize) - sdBox(p - stp.yyx, boxSize);

    return normalize(vec3(x, y, z));
}
