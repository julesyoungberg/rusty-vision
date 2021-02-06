mat4 createRotateAroundPointMatrix(vec3 point, vec3 rotationEuler);

#define CAMERA_MOVEMENT_SPEED -20.
#define CAMERA_INV_DISTANCE_MULTIPLIER 4.

void getRayData(const vec2 uv, const vec3 camPos, const vec3 lookAt,
                const float time, const vec3 worldUp, out vec3 rayOrigin,
                out vec3 rayDir) {
    rayOrigin = camPos;
    vec3 rayTargetPoint = vec3(0.0);

    // We want to move camera around center of the scene
    float cameraAngle = time * CAMERA_MOVEMENT_SPEED;
    mat4 rotateCameraMatrix =
        createRotateAroundPointMatrix(vec3(0.0), cameraAngle * worldUp);
    rayOrigin = (rotateCameraMatrix * vec4(rayOrigin, 1.0)).xyz;

    vec3 cameraForward = normalize(rayTargetPoint - rayOrigin);
    vec3 cameraRight = normalize(cross(cameraForward, worldUp));
    vec3 cameraUp = normalize(cross(cameraRight, cameraForward));
    mat3 cameraMatrix = mat3(cameraRight, cameraUp, cameraForward);

    rayDir = normalize(cameraMatrix * vec3(uv, CAMERA_INV_DISTANCE_MULTIPLIER));
}

void getRayData(const vec2 uv, const vec3 camPos, const vec3 lookAt,
                const float time, out vec3 rayOrigin, out vec3 rayDir) {
    getRayData(uv, camPos, lookAt, time, vec3(0, 1, 0), rayOrigin, rayDir);
}
