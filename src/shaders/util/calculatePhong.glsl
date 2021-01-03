/**
 * To use the following must be defined:
 * - MATERIAL_AMBIENT_STRENGTH
 * - MATERIAL_DIFFUSE_STRENGTH
 * - MATERIAL_SPECILAR_STRENGTH
 * - MATERIAL_SHININESS
 */
vec3 calculatePhong(const vec3 position, const vec3 normal, const vec3 eyePos,
                    const vec3 lightPos, const vec3 color) {
    vec3 rayDir = normalize(position - eyePos);
    float ambientValue = MATERIAL_AMBIENT_STRENGTH;

    vec3 lightDir = normalize(lightPos - position);
    float ndotl = max(dot(normal, lightDir), 0.);
    float diffuseValue = MATERIAL_DIFFUSE_STRENGTH * ndotl;

    vec3 reflectDir = reflect(lightDir, normal);
    float vdotr = max(dot(rayDir, reflectDir), 0.);
    float specularValue =
        MATERIAL_SPECULAR_STRENGTH * pow(vdotr, MATERIAL_SHININESS);

    vec3 resultColor = (ambientValue + diffuseValue + specularValue) * color;
    return resultColor;
}
