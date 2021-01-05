vec3 castRay(const vec2 uv, const vec3 camPos, const vec3 lookAt, const vec3 camUp) {
    vec3 camForward = normalize(lookAt - camPos);
    vec3 camRight = normalize(cross(camForward, camUp));
    mat3 camMatrix = mat3(camRight, camUp, camForward);
    return normalize(camMatrix * vec3(uv, FRAME_OF_VIEW));
}
