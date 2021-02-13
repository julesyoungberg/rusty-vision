#version 450

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 frag_color;

layout(set = 0, binding = 0) uniform GeneralUniforms {
    vec2 mouse;
    vec2 resolution;
    float time;
};

layout(set = 1, binding = 0) uniform sampler image_sampler;
layout(set = 1, binding = 1) uniform texture2D image1;
layout(set = 1, binding = 2) uniform texture2D image2;
layout(set = 1, binding = 2) uniform ImageUniforms {
    vec2 image1_size;
    vec2 image2_size;
};

#define PI 3.14159265359

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
    st *= 0.1;

    float x = st.x;
    float y = st.y;

    float r = length(st);
    float a = atan(y, x);

    float u = 0.02 * y + 0.03 * cos(a * 3 + time * 0.1) / (r * 0.5 * sin(time * 0.01));
    float v = 0.02 * x + 0.03 * sin(a * 3 - time * 0.1) / r;
    
    vec2 coord = vec2(u, v) * 0.5 + 0.5;
    vec3 color = texture(sampler2D(image1, image_sampler), mod(coord, 1.0)).xyz;
    
	frag_color = vec4(color, 1.0);
}
