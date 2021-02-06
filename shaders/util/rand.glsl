float rand(float n) { return fract(n * 1183.5437 + .42); }

float rand(vec2 co) {
    return fract(sin(dot(co.xy, vec2(12.9898, 78.233))) * 43758.5453);
}

float rand(vec3 co) {
    return fract(sin(dot(co, vec3(27.17, 112.61, 57.53))) * 43758.5453);
}

vec2 rand2(vec2 p) {
    return fract(
        sin(vec2(dot(p, vec2(127.1, 311.7)), dot(p, vec2(269.5, 183.3)))) *
        43758.5453);
}

vec3 rand3(vec3 p) {
    return fract(sin(vec3(dot(p, vec3(127.1, 311.7, 264.9)),
                          dot(p, vec3(269.5, 183.3, 491.5)),
                          dot(p, vec3(27.17, 112.61, 57.53)))) *
                 43758.5453);
}
