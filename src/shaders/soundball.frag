#version 450

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 frag_color;

layout(set = 0, binding = 0) uniform GeneralUniforms {
    vec2 resolution;
    float time;
};

layout(set = 1, binding = 0) uniform AudioUniforms {
    float dissonance;
    float energy;
    float loudness;
    float noisiness;
    float onset;
    float pitch;
    float rms;
    float spectralCentroid;
    float spectralComplexity;
    float spectralContrast;
    float tristimulus1;
    float tristimulus2;
    float tristimulus3;
};

layout(set = 2, binding = 0) uniform CameraUniforms {
    float cameraPosX;
    float cameraPosY;
    float cameraPosZ;
    float cameraTargetX;
    float cameraTargetY;
    float cameraTargetZ;
    float cameraUpX;
    float cameraUpY;
    float cameraUpZ;
};

#define NUM_STEPS 50
#define MIN_HIT_DISTANCE 0.01
#define MAX_TRACE_DISTANCE 20.0
#define FRAME_OF_VIEW 1.0

//@import primitives/sdSphere
//@import util/calculateNormal
//@import util/calculateSoftShadow
//@import util/castRay
//@import util/getSurfaceColor
//@import util/rayMarch

float floorDist(in vec3 p) { return p.y + 1.8; }

float sdSphere(in vec3 point, in vec3 center, float radius);

float distFromNearest(in vec3 p) {
    float t = sin(time * 0.5) * 2.0;
    float displacement = sin(6.0 * p.x) * sin(8.0 * p.y) * sin(5.0 * p.z * t + time * 0.5) * 0.25;
    float sphere1 = sdSphere(p, vec3(0), 1.0) + displacement;

    return min(sphere1, floorDist(p));
}

float calculateSoftShadow(vec3 position, vec3 lightPos, float k);

vec3 floorColor(in vec3 position, in vec3 normal, in vec3 eyePos, in vec3 lightPos) {
    return vec3(0.97) * calculateSoftShadow(position, lightPos, 30.0);
}

vec3 sphereColor(in vec3 position, in vec3 normal, in vec3 eyePos, in vec3 lightPos) {
    vec3 lightDir = normalize(lightPos - position);
    vec3 tristimulus = vec3(tristimulus1, tristimulus2, tristimulus3);

    vec3 ambientColor = tristimulus * 0.3;

    float diffuse = max(0.0, dot(normal, lightDir));
    vec3 diffuseColor = tristimulus * diffuse;

    const float specularStrength = 0.5;
    const float shininess = 128.0;
    vec3 eyeDir = normalize(eyePos - position);
    vec3 reflected = reflect(-lightDir, normal);
    float specular = pow(max(dot(eyeDir, reflected), 0.0), shininess) * specularStrength;
    vec3 specularColor = tristimulus * specular;

    return ambientColor + diffuseColor + specularColor;
}

vec3 calculateColor(in vec3 position, in vec3 normal, in vec3 eyePos) {
    const vec3 lightPos = vec3(3.0, 10.0, 8.0);

    if (floorDist(position) < MIN_HIT_DISTANCE) {
        return floorColor(position, normal, eyePos, lightPos);
    }

    return sphereColor(position, normal, eyePos, lightPos);
}

vec3 castRay(const vec2 uv, const vec3 camPos, const vec3 lookAt, const vec3 camUp);
vec3 getSurfaceColor(in vec3 origin, in vec3 rayDir, in vec3 bg);

void main() {
    vec2 st = uv;
    st.x *= resolution.x / resolution.y;

    vec3 camPos = vec3(cameraPosX, cameraPosY, cameraPosZ);
    vec3 camTarget = vec3(cameraTargetX, cameraTargetY, cameraTargetZ);
    vec3 camUp = vec3(cameraUpX, cameraUpY, cameraUpZ);
    const float zoom = 1.0;

    vec3 rayDir = castRay(st, camPos, camTarget, camUp);
    vec3 color = getSurfaceColor(camPos, rayDir, vec3(1.0));

    frag_color = vec4(color, 1);
}
