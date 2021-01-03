float sdSphere(in vec3 point, in vec3 center, float radius) {
    return length(point - center) - radius;
}

float sdSphere(in vec3 point, float radius) {
    return sdSphere(point, vec3(0), radius);
}
