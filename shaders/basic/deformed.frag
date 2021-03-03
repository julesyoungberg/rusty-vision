#version 450

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 frag_color;

layout(set = 0, binding = 0) uniform GeneralUniforms {
    vec2 mouse;
    vec2 resolution;
    float time;
    int mouse_down;
};

// https://www.shadertoy.com/view/Xl2yDW
float triangle(in vec2 p) {
    const float k = sqrt(3.0);
    p.x = abs(p.x) - 1.0;
    p.y = p.y + 1.0 / k;

    if (p.x + k * p.y > 0.0) {
        p = vec2(p.x - k * p.y, -k * p.x - p.y) / 2.0;
    }

    p.x -= clamp(p.x, -2.0, 0.0);
    return -length(p) * sign(p.y);
}

// https://www.iquilezles.org/www/articles/deform/deform.htm
// some things to try
// u = x*cos(2*r) - y*sin(2*r)
// v = y*cos(2*r) + x*sin(2*r)
// u = 0.3/(r+0.5*x)
// v = 3*a/pi
// u = 0.02*y+0.03*cos(a*3)/r
// v = 0.02*x+0.03*sin(a*3)/r
// u = 0.1*x/(0.11+r*0.5)
// v = 0.1*y/(0.11+r*0.5)
// u = 0.5*a/pi
// v = sin(7*r)
// u = r*cos(a+r)
// v = r*sin(a+r)
// u = 1/(r+0.5+0.5*sin(5*a))
// v = a*3/pi
// u = x/abs(y)
// v = 1/abs(y)

void main() {
    vec2 st = uv * resolution / resolution.y;
    st *= 2.0;

    float x = st.x;
    float y = st.y;

    float d = length(st);
    float a = atan(y, x);

    float u = x * cos(2.0 * d + time) - y * sin(2.0 * d);
    float v = y * cos(2.0 * d) + x * sin(2.0 * d - time);
    
	float dist = triangle(vec2(u, v));
    
    // basic shape
    vec3 color = mix(vec3(1.0, 1.0, 0), vec3(1.0, 0.0, 1.0), sign(dist));
	// dark fade
    color *= 1.0 - exp(-2.0 * abs(dist));
	// lines
    color = mix(color, vec3(0.0), 0.8 + 0.2 * cos(80.0 * dist));
	// triangle outline
    color = mix(color, vec3(1.0), 1.0 - smoothstep(0.0, 0.02, abs(dist)));
    
	frag_color = vec4(color, 1.0);
}
