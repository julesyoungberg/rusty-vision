float distFromNearest(in vec3 p);

vec3 calculateNormal(in vec3 point) {
    const vec3 stp = vec3(1e-5, 0, 0);

    float x =
        distFromNearest(point + stp.xyy) - distFromNearest(point - stp.xyy);
    float y =
        distFromNearest(point + stp.yxy) - distFromNearest(point - stp.yxy);
    float z =
        distFromNearest(point + stp.yyx) - distFromNearest(point - stp.yyx);

    return normalize(vec3(x, y, z));
}
