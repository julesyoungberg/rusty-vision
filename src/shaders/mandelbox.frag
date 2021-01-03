#version 450
precision highp float;

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 fragColor;

// // uniform vec3 moRotation;
// // uniform vec2 mousePosition;
// // uniform vec3 paletteColor1;
// // uniform vec3 paletteColor2;
// // uniform vec3 paletteColor3;
// // uniform vec3 shapeColor;
// // uniform vec3 shapeRotation;
layout(set = 0, binding = 0) uniform Uniforms {
    int colorMode;
    bool drawFloor;
    float fogDist;
    float quality;
    vec2 resolution;
    float shapeColorR;
    float shapeColorG;
    float shapeColorB;
    float time;
    float paletteColor1R;
    float paletteColor1G;
    float paletteColor1B;
    float paletteColor2R;
    float paletteColor2G;
    float paletteColor2B;
    float paletteColor3R;
    float paletteColor3G;
    float paletteColor3B;
};

// ray marching
#define FRAME_OF_VIEW 1.0
#define MAX_RAY_LENGTH 50.0
#define MAX_TRACE_DISTANCE 50.0
#define MIN_HIT_DISTANCE 0.02
#define NUM_STEPS 512
#define RAY_PUSH 0.05

// shading
#define LIGHT_POS vec3(20.0, 10.0, 20.0)
#define REFLECTIVITY 0.3
#define SHADOW_INTENSITY 0.9
#define SHADOW_FACTOR 128.0
#define MATERIAL_SHININESS 4.
#define MATERIAL_AMBIENT_STRENGTH 0.04
#define MATERIAL_DIFFUSE_STRENGTH 0.8
#define MATERIAL_SPECULAR_STRENGTH 0.6

// Scene
#define FLOOR_FADE_START 25.
#define FLOOR_FADE_END 50.
#define FLOOR_LEVEL -6.0

#define EPSILON 1e-5

//@import util/calculateAmbientOcclusion
//@import util/calculateFloorDist
//@import util/calculateNormal
//@import util/calculatePhong
//@import util/calculateReflectionsWithTrap
//@import util/calculateShadow
//@import util/castRay
//@import util/folding
//@import util/getRayData
//@import util/getUV
//@import util/hash
//@import util/marchRay
//@import util/marchRayWithTrap
//@import util/rayMarchWithTrap
//@import util/rotate

vec3 getBackgroundColor(const vec2 st) {
    return vec3(0) * smoothstep(1.0, 0.0, abs(0.5 - st.y));
}

mat4 createRotationMatrix(vec3 rotationEuler);
vec3 rotateVec(in vec3 v, in mat4 m);
vec3 foldBox(const vec3 z, const float limit);
vec4 foldSphere(const vec4 z, const float dotProduct, const float radius);

float sdMandelbox(const vec3 pos, const int iterations, out vec3 orbitTrap) {
    float scale = 3.0;
    vec3 offset = pos;
    vec3 z = pos;
    float dr = 1.0;
    float radius = 0.25;
    float R1 = abs(scale - 1.0);
    float R2 = pow(abs(scale), float(1 - iterations));
    vec4 temp;

    orbitTrap = vec3(1e20);

    vec3 moRotation = vec3(0);
    mat4 rotationMatrix = createRotationMatrix(moRotation);

    for (int i = 0; i < iterations; i++) {
        z = rotateVec(z, rotationMatrix);
        z = foldBox(z, 1.0);
        float r2 = dot(z, z);

        temp = foldSphere(vec4(z, dr), r2, radius);
        z = temp.xyz;
        dr = temp.w;

        z = z * scale / radius + offset;
        dr = dr * abs(scale) / radius + 1.0;

        orbitTrap.x = min(pow(abs(z.z), 0.1), orbitTrap.x);
        orbitTrap.y = min(abs(z.x) - 0.15, orbitTrap.y);
        orbitTrap.z = min(r2, orbitTrap.z);
	}

	return (length(z) - R1) / dr - R2;
}

float shapeDist(in vec3 pos, out vec3 orbitTrap) {
    vec3 shapeRotation = vec3(0);
    mat4 rot = createRotationMatrix(shapeRotation);
    vec3 p = (rot * vec4(pos, 1.)).xyz;
    return sdMandelbox(p, 10, orbitTrap);
}

float distFromNearest(in vec3 p, out vec3 trap) {
    return shapeDist(p, trap);
}

float distFromNearest(in vec3 p) {
    vec3 dummyTrap;
    return distFromNearest(p, dummyTrap);
}

vec3 calculatePhong(const vec3 position, const vec3 normal, const vec3 eyePos,
                    const vec3 lightPos, const vec3 color);

float calculateShadow(const vec3 position, const vec3 normal,
                      const vec3 lightPos);

vec3 calculateColor(in vec3 position, in vec3 normal, in vec3 eyePos, in vec3 trap) {
    vec3 color = vec3(shapeColorR, shapeColorG, shapeColorB);

    if (colorMode == 0) {
        vec3 paletteColor1 = vec3(paletteColor1R, paletteColor1G, paletteColor1B);
        vec3 paletteColor2 = vec3(paletteColor2R, paletteColor2G, paletteColor2B);
        vec3 paletteColor3 = vec3(paletteColor3R, paletteColor3G, paletteColor3B);
        color = paletteColor1 * clamp(pow(trap.x, 20.0), 0.0, 1.0);
        color += paletteColor2 * clamp(pow(trap.y, 20.0), 0.0, 1.0);
        color += paletteColor3 * clamp(pow(trap.z, 20.0), 0.0, 1.0);
    }

    color = calculatePhong(position, normal, eyePos, LIGHT_POS, color);
    color *= calculateShadow(position, normal, LIGHT_POS);
    return color;
}

vec2 hash(float p);
vec2 getUV(in vec2 coord, in vec2 res);
void getRayData(const vec2 uv, const vec3 camPos, const vec3 lookAt,
                const float time, out vec3 rayOrigin, out vec3 rayDir);
float marchRayWithTrap(const vec3 rayOrg, const vec3 rayDir,
                       const float startDist, out vec3 trap);
float calculateFloorDist(const vec3 rayOrigin, const vec3 rayDir,
                         const float level);
vec3 calculateReflectionsWithTrap(in vec3 position, in vec3 normal,
                                  in vec3 color, in vec3 eyePos, in vec3 bg);
vec3 calculateNormal(in vec3 point);

void main() {
    vec2 st = uv * resolution.x / resolution.y;
    const vec3 camPos = vec3(20.0, 3.0, 20.0);
    const vec3 lookAt = vec3(0.0);
    const float zoom = 1.0;

    vec3 finalColor = vec3(0.0);
    vec2 currentUV = st;
    vec3 backgroundColor;
    vec3 rayOrigin;
    vec3 rayDir;
    float d = quality;
    float numSubPixels = pow(d, 2.0);

    for (float i = 1.0; i <= numSubPixels; i += 1.0) {
        float x = mod(i - 1.0, d);
        float y = mod(floor(i / d), d);
        vec2 jitter = hash(i) / d;
        jitter.x += x / d;
        jitter.y += y / d;

        currentUV = getUV(st * resolution + jitter, resolution);
        getRayData(currentUV, camPos, lookAt, time, rayOrigin, rayDir);
        backgroundColor = getBackgroundColor(currentUV);

        vec3 trap;
        float dist = marchRayWithTrap(rayOrigin, rayDir, 0.0, trap);
        vec3 lightPos = LIGHT_POS;
        vec3 color = vec3(1.0);
        bool isFloor = false;
        vec3 surfacePos, surfaceNorm;
        if (dist < 0.0) {
            if (drawFloor) {
                dist = calculateFloorDist(rayOrigin, rayDir, FLOOR_LEVEL);
                if (dist >= 0.0) {
                    isFloor = true;
                    surfacePos = rayOrigin + rayDir * dist;
                    surfaceNorm = vec3(0, 1, 0);
                    color = vec3(1.0);
                    color = calculatePhong(surfacePos, surfaceNorm, rayOrigin, lightPos, color);
                    color *= calculateShadow(surfacePos, surfaceNorm, lightPos);
                    color = calculateReflectionsWithTrap(surfacePos, surfaceNorm, color, rayOrigin, vec3(0.0));
                }
            } else {
                dist = fogDist;
            }
        } else {
            surfacePos = rayOrigin + rayDir * dist;
            surfaceNorm = calculateNormal(surfacePos);
            color = calculateColor(surfacePos, surfaceNorm, rayOrigin, trap);
        }

        float backgroundBlend = smoothstep(FLOOR_FADE_START, FLOOR_FADE_END, dist);
        color = mix(color, backgroundColor, backgroundBlend);
        color = mix(color, vec3(0.5), pow(dist / fogDist, 2.0));
        finalColor = mix(finalColor, color, 1.0 / i);
    }

    fragColor = vec4(pow(finalColor, vec3(1. / 2.2)), 1);
}
