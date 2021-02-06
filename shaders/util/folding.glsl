// http://www.fractalforums.com/sierpinski-gasket/kaleidoscopic-(escape-time-ifs)/?PHPSESSID=f78f6a00e59b17e8798bceb6832bea64
vec3 foldTetrahedron(const vec3 pos) {
    vec3 p = pos;
    if (p.x + p.y < 0.0) {
        p.xy = -p.yx;
    }
    if (p.x + p.z < 0.0) {
        p.xz = -p.zx;
    }
    if (p.y + p.z < 0.0) {
        p.yz = -p.zy;
    }
    return p;
}

vec3 foldTetrahedronFull(const vec3 pos) {
    vec3 p = foldTetrahedron(pos);
    if (p.x < p.y) {
        p.xy = p.yx;
    }
    if (p.x < p.z) {
        p.xz = p.zx;
    }
    if (p.y < p.z) {
        p.yz = p.zy;
    }
    return p;
}

vec3 foldCube(const vec3 pos) { return abs(pos); }

vec3 foldOctahedral(const vec3 pos) {
    vec3 p = pos;
    if (p.x < p.y) {
        p.xy = p.yx;
    }
    if (p.x + p.y < 0.0) {
        p.xy = -p.yx;
    }
    if (p.x < p.z) {
        p.xz = p.zx;
    }
    if (p.x + p.z < 0.0) {
        p.xz = -p.zx;
    }
    return p;
}

vec3 foldOctahedralFull(const vec3 pos) {
    vec3 p = abs(pos);
    if (p.x - p.y < 0.0) {
        p.xy = p.yx;
    }
    if (p.x - p.z < 0.0) {
        p.xz = p.zx;
    }
    if (p.y - p.z < 0.0) {
        p.yz = p.zy;
    }
    return p;
}

vec3 foldBox(const vec3 z, const float limit) {
    return clamp(z, -limit, limit) * 2.0 - z;
}

vec4 foldSphere(const vec4 z, const float dotProduct, const float radius) {
    float k = clamp(max(radius / dotProduct, radius), 0.0, 1.0);
    return z * k;
}
