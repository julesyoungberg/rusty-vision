#version 450
precision highp float;

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 fragColor;

layout(set = 0, binding = 0) uniform GeneralUniforms {
    vec2 mouse;
    vec2 resolution;
    float time;
    int mouse_down;
};

layout(set = 1, binding = 0) uniform sampler spectrum_sampler;
layout(set = 1, binding = 1) uniform texture2D spectrum;

// ray marching
#define FRAME_OF_VIEW 5.0
#define MAX_RAY_LENGTH 30.0
#define MAX_TRACE_DISTANCE 30.0
#define MIN_HIT_DISTANCE 0.001
#define NUM_STEPS 100
#define RAY_PUSH 0.02

// shading
#define LIGHT_POS vec3(5.0, 10.0, 5.0)
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
#define FLOOR_LEVEL -2.0
#define FOG_DIST 15

#define EPSILON 1e-5

//@import util/calculateFloorDist
//@import util/calculateNormal
//@import util/calculatePhong
//@import util/calculateReflectionsWithTrap
//@import util/calculateShadow
//@import util/folding
//@import util/marchRayWithTrap
//@import util/rotate

vec3 getBackgroundColor(const vec2 st) {
    return vec3(0) * smoothstep(1.0, 0.0, abs(0.5 - st.y));
}

mat4 createRotationMatrix(vec3 rotationEuler);
vec3 rotateVec(in vec3 v, in mat4 m);

float sdMandelbulb(const vec3 pos, const int iterations, 
                   const float bailout, out vec3 orbitTrap) {
    float thresh = length(pos) - 1.2;
    if (thresh > 0.2) {
        return thresh;
    }

    vec3 z = pos;
    float dr = 1.0;
    float r = 0.0;

    orbitTrap = vec3(1e20);

    float power = 8.0 ; //+ sin(time * 0.1) * 4.0;

    for (int i = 0; i < iterations; i++) {
        r = length(z);
        if (r > bailout) {
            break;
        }

        // convert to polar coordinates
        float theta = acos(z.z / r) - time * 0.1;
        float phi = atan(z.y, z.x);
        dr = pow(r, power - 1.0) * power * dr + 1.0;

        // scale and rotate the point
        float zr = pow(r, power);
        theta = theta * power;
        phi = phi * power;

        // convert back to cartesian coordinates
        z = zr * vec3(sin(theta) * cos(phi), sin(phi) * sin(theta), cos(theta));
        z += pos;

        orbitTrap.x = min(pow(abs(z.z), 0.1), orbitTrap.x);
        orbitTrap.y = min(abs(z.x) - 0.15, orbitTrap.y);
        orbitTrap.z = min(length(z), orbitTrap.z);
	}

	return 0.5 * log(r) * r / dr;
}

float shapeDist(in vec3 pos, out vec3 orbitTrap) {
    return sdMandelbulb(pos, 10, 2.0, orbitTrap);
}

float distFromNearest(in vec3 p, out vec3 trap) {
    return shapeDist(p, trap);
}

float distFromNearest(in vec3 p) {
    vec3 dummyTrap;
    return shapeDist(p, dummyTrap);
}

vec3 calculatePhong(const vec3 position, const vec3 normal, const vec3 eyePos,
                    const vec3 lightPos, const vec3 color);
float calculateShadow(const vec3 position, const vec3 normal,
                      const vec3 lightPos);

vec3 calculateColor(in vec3 position, in vec3 normal, in vec3 eyePos, in vec3 trap) {
    vec3 paletteColor1 = mod(vec3(
        texture(sampler2D(spectrum, spectrum_sampler), vec2(0.77, 0)).x * 1.9,
        texture(sampler2D(spectrum, spectrum_sampler), vec2(0.44, 0)).x * 0.7,
        texture(sampler2D(spectrum, spectrum_sampler), vec2(0.11, 0)).x * 0.3
    ) + vec3(1.2, 3.7, 7.5), 1);
    vec3 paletteColor2 = mod(vec3(
        texture(sampler2D(spectrum, spectrum_sampler), vec2(0.22, 0)).x * 0.4,
        texture(sampler2D(spectrum, spectrum_sampler), vec2(0.55, 0)).x,
        texture(sampler2D(spectrum, spectrum_sampler), vec2(0.88, 0)).x * 2.1
    ) + vec3(0.3, 10.6, 6.9), 1);
    vec3 paletteColor3 = mod(vec3(
        texture(sampler2D(spectrum, spectrum_sampler), vec2(0.33, 0)).x * 0.5,
        texture(sampler2D(spectrum, spectrum_sampler), vec2(0.66, 0)).x * 1.6,
        texture(sampler2D(spectrum, spectrum_sampler), vec2(0.99, 0)).x * 2.3
    ) + vec3(0.8, 5.1, 13.9), 1);
    
    vec3 color = paletteColor1 * clamp(pow(trap.x, 20.0), 0.0, 1.0);
    color += paletteColor2 * clamp(pow(trap.y, 20.0), 0.0, 1.0);
    color += paletteColor3 * clamp(pow(trap.z, 20.0), 0.0, 1.0);

    color = calculatePhong(position, normal, eyePos, eyePos, color);
    color *= calculateShadow(position, normal, eyePos);
    return color;
}

vec3 castRay(const vec2 uv, const vec3 camPos, const vec3 lookAt, const vec3 camUp);
float marchRayWithTrap(const vec3 rayOrg, const vec3 rayDir,
                       const float startDist, out vec3 trap);
float calculateFloorDist(const vec3 camPos, const vec3 rayDir,
                         const float level);
vec3 calculateReflectionsWithTrap(in vec3 position, in vec3 normal,
                                  in vec3 color, in vec3 eyePos, in vec3 bg);
vec3 calculateNormal(in vec3 point);

void main() {
    float freq = 80.0 + time;
    vec3 camPos = vec3(3.0 * cos(0.1 * 0.125 * freq) * sin(0.1 * 0.5 * freq), sin(0.1 * freq), 2.0 * cos(0.1 * 0.5 * freq));
    const vec3 camTarget = vec3(0);
    const float fov = 90.0 * 3.141592 / 180.0;
    float h = 1.0;
    vec3 camDir = normalize(camTarget - camPos);
    vec3 camSide = normalize(cross(vec3(0,1,0), camDir));
    vec3 camUp = normalize(cross(camDir, camSide));

    vec2 currentUV = uv;
    currentUV.x *= resolution.x / resolution.y;

    vec3 rayDir = normalize(currentUV.x * h * camSide + currentUV.y * h * camUp + camDir - camPos);
    vec3 backgroundColor = getBackgroundColor(currentUV);

    vec3 trap;
    float dist = marchRayWithTrap(camPos, rayDir, 0.0, trap);
    vec3 color = vec3(1.0);
    bool isFloor = false;
    vec3 surfacePos, surfaceNorm;
    if (dist < 0.0) {
        dist = FOG_DIST;
    } else {
        surfacePos = camPos + rayDir * dist;
        surfaceNorm = calculateNormal(surfacePos);
        color = calculateColor(surfacePos, surfaceNorm, camPos, trap);
    }

    float backgroundBlend = smoothstep(FLOOR_FADE_START, FLOOR_FADE_END, dist);
    color = mix(color, backgroundColor, backgroundBlend);
    color = mix(color, vec3(0.5), pow(dist / FOG_DIST, 2.0));

    fragColor = vec4(pow(color, vec3(1. / 2.2)), 1);
}
