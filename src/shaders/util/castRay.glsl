vec3 castRay(in vec2 uv, in vec3 camPos, in vec3 lookAt, in float zoom) {
    vec3 f = normalize(lookAt - camPos);
    vec3 r = cross(vec3(0.0, 1.0, 0.0), f);
    vec3 u = cross(f, r);
    vec3 c = camPos + f * zoom;
    vec3 i = c + uv.x * r + uv.y * u;
    vec3 dir = i - camPos;
    return normalize(dir);
}
