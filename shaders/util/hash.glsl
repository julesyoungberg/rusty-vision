#define HASHSCALE3 vec3(443.897, 441.423, 437.195)
//----------------------------------------------------------------------------------------
//  2 out, 1 in...
vec2 hash(float p) {
    vec3 p3 = fract(vec3(p) * HASHSCALE3);
    p3 += dot(p3, p3.yzx + 19.19);
    return fract(vec2((p3.x + p3.y) * p3.z, (p3.x + p3.z) * p3.y));
}
