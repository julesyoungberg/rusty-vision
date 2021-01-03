float dot2(in vec2 v) { return dot(v, v); }
float dot2(in vec3 v) { return dot(v, v); }
float ndot(in vec2 a, in vec2 b) { return a.x * b.x - a.y * b.y; }

float sdUnion(float a, float b) { return min(a, b); }
float sdIntersect(float a, float b) { return max(a, b); }
float sdSubtract(float a, float b) { return sdIntersect(a, -b); }
float sdLerp(float a, float b, float t) { return (1.0 - t) * a + t * b; }
