/*{
    "DESCRIPTION": "Bokeh.",
    "CREDIT": "by julesyoungberg",
    "ISFVSN": "2.0",
    "CATEGORIES": [ "Blur" ],
    "INPUTS": [
        {
            "NAME": "inputImage",
            "TYPE": "image"
        },
        {
            "NAME": "radius",
            "TYPE": "float",
            "MIN": -2.0,
            "MAX": 2.0,
            "DEFAULT": 0.5
        },
        {
            "NAME": "iterations",
            "TYPE": "int",
            "MIN": 1.0,
            "MAX": 1024,
            "DEFAULT": 512
        },
        {
            "NAME": "distortion_anamorphic",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 1.0,
            "DEFAULT": 0.6
        },
        {
            "NAME": "distortion_barrel",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 1.0,
            "DEFAULT": 0.6
        },
        {
            "NAME": "offset_x",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 1.0,
            "DEFAULT": 0.5
        },
        {
            "NAME": "offset_y",
            "TYPE": "float",
            "MIN": 0.0,
            "MAX": 1.0,
            "DEFAULT": 0.5
        }
    ],
    "PASSES": []
}*/

// Based on
// https://www.shadertoy.com/view/4stSRM

// The Golden Angle is (3.-sqrt(5.0))*PI radians
#define GOLDEN_ANGLE 2.39996323

vec3 image_color(in vec2 coord) {
    vec2 c = fract(coord);
    return IMG_NORM_PIXEL(inputImage, vec2(c.x, 1.0 - c.y)).rgb;
}

vec2 rotate(vec2 vector, float angle) {
    float s = sin(angle);
    float c = cos(angle);
    
    return vec2(c*vector.x-s*vector.y, s*vector.x+c*vector.y);
}

mat2 rotMatrix(float angle) {
    return mat2(cos(angle), sin(angle),
                    -sin(angle), cos(angle));
}

vec2 getDistOffset(vec2 uv, vec2 pxoffset) {
    vec2 tocenter = uv.xy + vec2(-offset_x, offset_y);
    vec3 prep = normalize(vec3(tocenter.y, -tocenter.x, 0.0));
    
    float angle = length(tocenter.xy) * 1.5 * distortion_barrel;
    vec3 oldoffset = vec3(pxoffset, 0.0);
    oldoffset.x *= 1.0 - distortion_anamorphic;
    
    vec3 rotated = oldoffset * cos(angle) + cross(prep, oldoffset) * sin(angle) + prep * dot(prep, oldoffset) * (1.0-cos(angle));
    
    return rotated.xy;
}

vec3 bokeh(vec2 uv) {
	vec3 acc = vec3(0.0);
	vec3 div = vec3(0.0);
    vec2 tocenter = uv.xy + vec2(-offset_x, offset_y);
    vec2 pixel = 1.0 / RENDERSIZE;
    float r = 1.0;
    vec2 vangle = vec2(0.0, radius);
    mat2 rot = rotMatrix(GOLDEN_ANGLE);
    
    float a = radius * 500.0;
    
	for (int j = 0; j < iterations; j++) {
        r += 1.0 / r;
	    vangle = rot * vangle;
        vec2 pos = getDistOffset(uv, pixel * (r - 1.0) * vangle);
        
        vec3 col = image_color(uv + pos);
        col = col * col * 1.5;
		vec3 bokeh = pow(col, vec3(9.0)) * a + 0.4;
		acc += col * bokeh;
		div += bokeh;
	}

	return acc / div;
}

void main() {
    vec2 st = isf_FragNormCoord;
    gl_FragColor = vec4(bokeh(st), 1.0);
}
