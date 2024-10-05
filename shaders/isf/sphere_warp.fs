
/*{
	"DESCRIPTION": "",
	"CREDIT": "https://editor.isf.video/shaders/62d5e32ead0a68001af8ab9e",
	"ISFVSN": "2",
	"CATEGORIES": [
		"XXX"
	],
	"INPUTS": [
		{
			"NAME": "inputImage",
			"TYPE": "image"
		},
		{
			"NAME": "warp",
			"TYPE": "float",
			"DEFAULT": 0.5,
			"MIN": 0.0,
			"MAX": 0.99
		},
		{
			"NAME": "blur",
			"TYPE": "float",
			"DEFAULT": 0.2,
			"MIN": 0.0,
			"MAX": 1.0
		},
		{
			"NAME": "center",
			"TYPE": "point2D",
			"DEFAULT": [
				0.5,
				0.5
			],
			"MIN": [
				0.0,
				0.0
			],
			"MAX": [
				1.0,
				1.0
			]
		}
	]
	
}*/

const float blurScaleMult = 100.0;

// Gaussian blur adapted from @mattdesl's implementation:
// https://github.com/Jam3/glsl-fast-gaussian-blur/blob/master/9.glsl
vec4 blurLookup(vec2 uv, vec2 resolution, vec2 direction) {
  vec4 color = vec4(0.0);
  vec2 off1 = vec2(1.3846153846) * direction;
  vec2 off2 = vec2(3.2307692308) * direction;
  color += IMG_NORM_PIXEL(inputImage, uv) * 0.2270270270;
  color += IMG_NORM_PIXEL(inputImage, uv + (off1 / resolution)) * 0.3162162162;
  color += IMG_NORM_PIXEL(inputImage, uv - (off1 / resolution)) * 0.3162162162;
  color += IMG_NORM_PIXEL(inputImage, uv + (off2 / resolution)) * 0.0702702703;
  color += IMG_NORM_PIXEL(inputImage, uv - (off2 / resolution)) * 0.0702702703;
  return color;
}

void main()	{
    // Calculate spheroid warp: paint pixels onto the inside of a spheroid,
    // intersected by the rectangular viewport. Resample pixels onto viewport.
    vec2 uv = isf_FragNormCoord.xy;
    
    // Directionality; calculations differ slightly left/right of center
    vec2 dirMultL = step(uv, center);
    vec2 dirMultR = 1.0 - dirMultL;

    // D: distance between the camera (spheroid center) & the viewport plane
    float D = -log(warp);
    // C: distance between the viewport center & the edge of viewport/spheroid
    vec2 C = dirMultL * center + dirMultR * (vec2(1.0) - center);
    // R: distance between the camera & the edge of the viewport/spheroid
	vec2 R = sqrt(C * C + D * D);
    // a: distance between the viewport center and the viewport uv
	vec2 a = dirMultL * (center - uv) + dirMultR * (uv - center);
    // b: distance between the viewport center and the spheroid projection uv
	vec2 b = R * sin(atan(a / D));
    // uvPrime: b remapped to the texture index space
	vec2 uvPrime = dirMultL * (center - b) + dirMultR * (center + b);
    // Passthrough when warp == 0, i.e. when D approaches infinity
    uvPrime = (warp == 0.0) ? uv : uvPrime;
	
	// Apply blur/echo in the direction of the warp
	vec2 direction = (uvPrime - uv) * blur * blurScaleMult;
	vec4 color = blurLookup(uvPrime, RENDERSIZE, direction);
	gl_FragColor = color;
}
