mat3 lookAt(in vec3 forward, in vec3 up) {
    vec3 fw = normalize(forward);
    vec3 rt = normalize(cross(fw, up));
    return mat3(rt, cross(rt, fw), fw);
}

mat3 lookAt(in vec3 fw) { return lookAt(fw, vec3(0, 1, 0)); }
