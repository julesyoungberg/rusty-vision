// #define PI = 3.1416

mat4 rotateX(in float angle) {
    return mat4(1.0, 0, 0, 0, 0, cos(angle), -sin(angle), 0, 0, sin(angle),
                cos(angle), 0, 0, 0, 0, 1);
}

mat4 rotateY(in float angle) {
    return mat4(cos(angle), 0, sin(angle), 0, 0, 1.0, 0, 0, -sin(angle), 0,
                cos(angle), 0, 0, 0, 0, 1);
}

mat4 rotateZ(in float angle) {
    return mat4(cos(angle), -sin(angle), 0, 0, sin(angle), cos(angle), 0, 0, 0,
                0, 1, 0, 0, 0, 0, 1);
}

vec3 rotateVec(in vec3 v, in mat4 m) {
    vec4 rotated = m * vec4(v, 1);
    return rotated.xyz;
}

mat4 createTranslationMatrix(vec3 position) {
    mat4 translationMatrix =
        mat4(vec4(1., 0., 0., 0.), vec4(0., 1., 0., 0.), vec4(0., 0., 1., 0.),
             vec4(position.x, position.y, position.z, 1.));
    return translationMatrix;
}

mat4 createRotationMatrix(vec3 rotationEuler) {
    // Input is in degrees, but to calculate everything property we need radians
    vec3 rotationTheta = rotationEuler * (3.1416 / 180.);

    vec3 cosTheta = cos(rotationTheta);
    vec3 sinTheta = sin(rotationTheta);

    mat4 rotateAroundXMatrix =
        mat4(vec4(1., 0., 0., 0.), vec4(0., cosTheta.x, sinTheta.x, 0.),
             vec4(0., -sinTheta.x, cosTheta.x, 0.), vec4(0., 0., 0., 1.));
    mat4 rotateAroundYMatrix =
        mat4(vec4(cosTheta.y, 0., -sinTheta.y, 0.), vec4(0., 1., 0., 0.),
             vec4(sinTheta.y, 0., cosTheta.y, 0.), vec4(0., 0., 0., 1.));
    mat4 rotateAroundZMatrix = mat4(vec4(cosTheta.z, sinTheta.z, 0., 0.),
                                    vec4(-sinTheta.z, cosTheta.z, 0., 0.),
                                    vec4(0., 0., 1., 0.), vec4(0., 0., 0., 1.));
    mat4 rotationMatrix =
        rotateAroundZMatrix * rotateAroundYMatrix * rotateAroundXMatrix;

    return rotationMatrix;
}

// Creates classic matrix which translates and then rotates
mat4 createTransformationMatrix(vec3 position, vec3 rotationEuler) {
    mat4 translationMatrix = createTranslationMatrix(position);
    mat4 rotationMatrix = createRotationMatrix(rotationEuler);
    mat4 transformationMatrix = rotationMatrix * translationMatrix;
    return transformationMatrix;
}

// Creates matrix which rotates around given point
mat4 createRotateAroundPointMatrix(vec3 point, vec3 rotationEuler) {
    // When rotating around a point we need to use two translation matrices
    mat4 translationMatrix = createTranslationMatrix(point);
    mat4 secTranslationMatrix = createTranslationMatrix(-point);
    mat4 rotationMatrix = createRotationMatrix(rotationEuler);
    return translationMatrix * rotationMatrix * secTranslationMatrix;
}
