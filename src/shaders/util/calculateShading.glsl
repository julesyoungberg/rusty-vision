float getShadowMultiplier(in vec3 position, in vec3 lightPos,
                          in float shadowConst, in float b);

vec3 calculateShading(in vec3 position, in vec3 normal, in vec3 eyePos,
                      in vec3 lightPos, in vec3 color, in vec3 specColor) {
    vec3 lightDir = normalize(lightPos - position);

    float diffuse = max(0.0, dot(normal, lightDir));
    vec3 diffuseColor = color * diffuse;

    const float specularStrength = 0.5;
    const float shininess = 64.0;
    vec3 eyeDir = normalize(eyePos - position);
    vec3 reflected = reflect(-lightDir, normal);
    float specular =
        pow(max(dot(eyeDir, reflected), 0.0), shininess) * specularStrength;
    vec3 specularColor = specColor * specular;

    vec3 finalColor = vec3(0.2) * 0.5 + diffuseColor + specularColor;
    finalColor *= getShadowMultiplier(position, lightPos, 30.0, 0.3);
    return finalColor;
}

vec3 calculateShading(in vec3 position, in vec3 normal, in vec3 eyePos,
                      in vec3 lightPos, in vec3 color) {
    return calculateShading(position, normal, eyePos, lightPos, color, color);
}
