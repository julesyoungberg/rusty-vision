#version 450
precision highp float;

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 fragColor;

layout(set = 0, binding = 0) uniform Uniforms {
    int colorMode;
    int drawFloor;
    float fogDist;
    float quality;
    vec2 resolution;
    float time;
    float color1R;
    float color1G;
    float color1B;
    float color2R;
    float color2G;
    float color2B;
    float color3R;
    float color3G;
    float color3B;
    float cameraPosX;
    float cameraPosY;
    float cameraPosZ;
    float cameraTargetX;
    float cameraTargetY;
    float cameraTargetZ;
    float cameraUpX;
    float cameraUpY;
    float cameraUpZ;
    float rotation1X;
    float rotation1Y;
    float rotation1Z;
    float offset1X;
    float offset1Y;
    float offset1Z;
};

// ray marching
#define FRAME_OF_VIEW 6.0
#define MAX_RAY_LENGTH 50.0
#define MAX_TRACE_DISTANCE 50.0
#define MIN_HIT_DISTANCE 0.002
#define NUM_STEPS 512
#define RAY_PUSH 0.004

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

//@import util/calculateFloorDist
//@import util/calculateNormal
//@import util/calculatePhong
//@import util/calculateReflectionsWithTrap
//@import util/calculateShadow
//@import util/folding
//@import util/getUV
//@import util/hash
//@import util/marchRayWithTrap
//@import util/rotate

vec3 castRay(const vec2 uv, const vec3 camPos, const vec3 lookAt, const vec3 camUp) {
    vec3 camForward = normalize(lookAt - camPos);
    vec3 camRight = normalize(cross(camForward, camUp));
    mat3 camMatrix = mat3(camRight, camUp, camForward);
    return normalize(camMatrix * vec3(uv, FRAME_OF_VIEW));
}

vec3 getBackgroundColor(const vec2 st) {
    return vec3(0) * smoothstep(1.0, 0.0, abs(0.5 - st.y));
}

mat4 createRotationMatrix(vec3 rotationEuler);
vec3 rotateVec(in vec3 v, in mat4 m);
vec3 foldBox(const vec3 z, const float limit);
vec4 foldSphere(const vec4 z, const float dotProduct, const float radius);

float sdMandelbox(const vec3 pos, const int iterations, out vec3 orbitTrap) {
    float scale = 3.0;
    vec3 offset = pos + vec3(offset1X, offset1Y, offset1Z);
    vec3 z = pos;
    float dr = 1.0;
    float radius = 0.25;
    float R1 = abs(scale - 1.0);
    float R2 = pow(abs(scale), float(1 - iterations));
    vec4 temp;

    orbitTrap = vec3(1e20);

    vec3 moRotation = vec3(rotation1X, rotation1Y, rotation1Z);
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
    return sdMandelbox(p, 8, orbitTrap);
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
    vec3 color = vec3(color1R, color1G, color1B);

    if (colorMode == 0) {
        vec3 paletteColor1 = vec3(color1R, color1G, color1B);
        vec3 paletteColor2 = vec3(color2R, color2G, color2B);
        vec3 paletteColor3 = vec3(color3R, color3G, color3B);
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
float marchRayWithTrap(const vec3 rayOrg, const vec3 rayDir,
                       const float startDist, out vec3 trap);
float calculateFloorDist(const vec3 rayOrigin, const vec3 rayDir,
                         const float level);
vec3 calculateReflectionsWithTrap(in vec3 position, in vec3 normal,
                                  in vec3 color, in vec3 eyePos, in vec3 bg);
vec3 calculateNormal(in vec3 point);

void main() {
    vec2 st = uv * resolution.x / resolution.y;
    vec3 camPos = vec3(cameraPosX, cameraPosY, cameraPosZ);
    vec3 lookAt = vec3(cameraTargetX, cameraTargetY, cameraTargetZ);
    vec3 camUp = vec3(cameraUpX, cameraUpY, cameraUpZ);
    const float zoom = 1.0;

    vec3 finalColor = vec3(0.0);
    vec2 currentUV = st;
    vec3 backgroundColor;
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
        rayDir = castRay(currentUV, camPos, lookAt, camUp);
        backgroundColor = getBackgroundColor(currentUV);

        vec3 trap;
        float dist = marchRayWithTrap(camPos, rayDir, 0.0, trap);
        vec3 lightPos = LIGHT_POS;
        vec3 color = vec3(1.0);
        bool isFloor = false;
        vec3 surfacePos, surfaceNorm;
        if (dist < 0.0) {
            if (drawFloor == 1) {
                dist = calculateFloorDist(camPos, rayDir, FLOOR_LEVEL);
                if (dist >= 0.0) {
                    isFloor = true;
                    surfacePos = camPos + rayDir * dist;
                    surfaceNorm = vec3(0, 1, 0);
                    color = vec3(1.0);
                    color = calculatePhong(surfacePos, surfaceNorm, camPos, lightPos, color);
                    color *= calculateShadow(surfacePos, surfaceNorm, lightPos);
                    color = calculateReflectionsWithTrap(surfacePos, surfaceNorm, color, camPos, vec3(0.0));
                }
            } else {
                dist = fogDist;
            }
        } else {
            surfacePos = camPos + rayDir * dist;
            surfaceNorm = calculateNormal(surfacePos);
            color = calculateColor(surfacePos, surfaceNorm, camPos, trap);
        }

        float backgroundBlend = smoothstep(FLOOR_FADE_START, FLOOR_FADE_END, dist);
        color = mix(color, backgroundColor, backgroundBlend);
        color = mix(color, vec3(0.5), pow(dist / fogDist, 2.0));
        finalColor = mix(finalColor, color, 1.0 / i);
    }

    fragColor = vec4(pow(finalColor, vec3(1. / 2.2)), 1);
}
