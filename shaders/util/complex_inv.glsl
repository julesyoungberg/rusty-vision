vec2 complex_inv(in vec2 z) {
    vec2 conjugate = vec2(z.x, -z.y);
    float denominator = dot(conjugate, conjugate);
    return conjugate / denominator;
}
