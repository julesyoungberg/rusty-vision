
/*{
	"DESCRIPTION": "",
	"CREDIT": "",
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
			"NAME": "boolInput",
			"TYPE": "bool",
			"DEFAULT": 1.0
		},
		{
			"NAME": "colorInput",
			"TYPE": "color",
			"DEFAULT": [
				0.0,
				0.0,
				1.0,
				1.0
			]
		},
		{
			"NAME": "flashInput",
			"TYPE": "event"
		},
		{
			"NAME": "floatInput",
			"TYPE": "float",
			"DEFAULT": 0.5,
			"MIN": 0.0,
			"MAX": 1.0
		},
		{
			"NAME": "longInputIsAPopUpButton",
			"TYPE": "long",
			"VALUES": [
				0,
				1,
				2
			],
			"LABELS": [
				"red",
				"green",
				"blue"
			],
			"DEFAULT": 1
		},
		{
			"NAME": "pointInput",
			"TYPE": "point2D",
			"DEFAULT": [
				0,
				0
			]
		}
	],
	"PASSES": [
		{
			"TARGET":"bufferVariableNameA",
			"WIDTH": "$WIDTH/16.0",
			"HEIGHT": "$HEIGHT/16.0"
		},
		{
			"DESCRIPTION": "this empty pass is rendered at the same rez as whatever you are running the ISF filter at- the previous step rendered an image at one-sixteenth the res, so this step ensures that the output is full-size"
		}
	]
	
}*/

void main()	{
	vec4		inputPixelColor;
	//	both of these are the same
	// inputPixelColor = IMG_THIS_PIXEL(inputImage);
	// inputPixelColor = IMG_PIXEL(inputImage, gl_FragCoord.xy);
	
	//	both of these are also the same
	// inputPixelColor = IMG_NORM_PIXEL(inputImage, isf_FragNormCoord.xy);
	// inputPixelColor = IMG_THIS_NORM_PIXEL(inputImage);
	
	vec2 uv = isf_FragNormCoord.xy;

    uv.y = 1.0 - uv.y;
	
	float freq = 15.0;
	float amount = 0.05;
	float speed = 1.0;
	
	vec2 center = vec2(0.5, 0.2);
	float radius = 0.3; 
	
	float dist = length(center - uv);
	
	vec2 inner = uv;
	inner.y += sin(uv.x * freq + TIME * speed) * amount;
	
	uv = mix(uv, inner, smoothstep(0.17, 0.1, dist));
	
	float outerFreq = 200.0;
	float outerAmount = 0.002;
	float outerSpeed = 2.0;
	
	vec2 outer = uv;
	outer.y += sin(uv.x * outerFreq + TIME * outerSpeed) * outerAmount;
	outer.x += cos(uv.y * outerFreq + TIME * outerSpeed) * outerAmount;
	
	uv = mix(uv, outer, smoothstep(0.2, 0.5, dist));
	
	float chromaAmount = 0.006 + sin(TIME * 1.8) * 0.005;
	
	float r = IMG_NORM_PIXEL(inputImage, uv + vec2(chromaAmount)).r;
	float g = IMG_NORM_PIXEL(inputImage, uv).g;
	float b = IMG_NORM_PIXEL(inputImage, uv - vec2(chromaAmount)).b;
	
	gl_FragColor = vec4(r, g, b, 1.0);
}
