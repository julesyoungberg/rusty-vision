#version 450

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 frag_color;

layout(set = 0, binding = 0) uniform GeneralUniforms {
    vec2 mouse;
    vec2 resolution;
    float time;
    int mouse_down;
};

layout(set = 1, binding = 0) uniform sampler webcam_sampler;
layout(set = 1, binding = 1) uniform utexture2D webcam;
layout(set = 1, binding = 2) uniform WebcamUniforms {
    vec2 video_size;
};

layout(set = 2, binding = 0) uniform sampler spectrum_sampler;
layout(set = 2, binding = 1) uniform texture2D spectrum;

// based on Webcam CRT by porglezomp
// https://www.shadertoy.com/view/MdlGRB
// and VCR Distortion by ryk
// https://www.shadertoy.com/view/ldjGzV

//@import util/noise
//@import util/pulse
//@import util/rand

float noise2(in vec2 x);
float noise3(in vec3 x);
float pulse(float c, float w, float x);
float rand21(vec2 p);

vec3 webcam_color(in vec2 coord) {
    return texture(usampler2D(webcam, webcam_sampler), fract(coord)).xyz / 255.0;
}

float get_spectrum(float i) {
    return texture(sampler2D(spectrum, spectrum_sampler), vec2(fract(i), 0)).x;
}

float ramp(float y, float start, float end) {
	float inside = step(start, y) - step(end, y);
	float fact = (y - start) / (end - start) * inside;
	return (1.0 - fact) * inside;
}

void main() {
    vec2 st = uv * 0.5 + 0.5;

    // bend the space
    vec2 disp = st - 0.5;
    disp *= sqrt(length(disp));
    st += disp * 1.5;
    st += 0.5;
    st *= 0.5;

    // return black outside the tv frame
    if (!(st.x > 0.0 && st.x < 1.0 && st.y > 0.0 && st.y < 1.0)) {
        frag_color = vec4(vec3(0.0), 1.0);
        float d = max(abs(st.x - 0.5), abs(st.y - 0.5)) - 0.5;
        frag_color += d * 0.1;
        return;
    }

    // save the bent coordinates from this point
    vec2 tv = st;

    // blend the bent coords with originals by sound
    float d = mix(0.5, 1.5, clamp(get_spectrum(0.1) * 0.5, 0.0, 1.0));
    st = mix(uv * 0.5 + 0.5, st, d);

    // apply shifting
    // the window targets a specific horizontal region 
    float window = 1.0 / (1.0 + 20.0 * (st.y - mod(time * 0.25, 1.0)) * (st.y - mod(time * 0.25, 1.0)));
    // start with high freq compound wave
    float x_shift = sin(st.y * 5.0 + time) / 50.0 * (1.0 + cos(time * 80.0));
    x_shift *= window; // concentrate the wave
    x_shift *= step(0.3, get_spectrum(0.3)); // flicker
    st.x += x_shift; // apply shift
    // start with jiggle compound wave
    float y_shift = 0.4 * sin(time) * sin(time * 20.0);
    y_shift += 0.1 * sin(time * 200.0 * cos(time)); // add fast flickerywave
    y_shift *= step(0.1, get_spectrum(0.6)); // flicker
    st.y += y_shift;

    // calculate each channel coord to get chromatic shift effect
    float dispersion = mix(0.001, 0.1, get_spectrum(0.3));
    vec2 str = st * (1.0 - dispersion) + vec2(dispersion * 0.5);
	vec2 stg = st;
	vec2 stb = st * (1.0 + dispersion) - vec2(dispersion * 0.5);

    // calculate noise effect
    float offset = noise2(vec2(0, st.y + time * 155.0));
    float distortion = mix(0.002, 0.008, get_spectrum(0.7));
    float noisestrength = mix(0.002, 0.008, get_spectrum(0.5));

    // get colors for each channel with noise
    float r = mix(webcam_color(str + offset * distortion).r, offset, noisestrength);
    float g = mix(webcam_color(stg + offset * distortion).g, offset, noisestrength);
    float b = mix(webcam_color(stb + offset * distortion).b, offset, noisestrength);
    vec3 color = vec3(r, g, b);

    // TV noise
    color += rand21(st) * get_spectrum(0.8);

    // add small TV noise stripes
    float stripes = sin(st.y * 300.0 + time * 20.0 + sin(time * 0.27) * 300.0);
    color = mix(color, vec3(0.8), stripes / 20.0);

    // add big TV noise stripes
    float n = rand21(st) * 0.5 + 0.5;
    float t = st.y + time * 0.5 + sin(time + sin(time * 0.63) * get_spectrum(0.4));
    color = mix(color, vec3(n), pulse(0.5, 0.05, fract(t)) * 0.4);

    // edge fade
    tv *= 2.0;
    tv -= 1.0;
    float v = (1.0 - exp((abs(tv.x) - 1.0) * 3.0)) * (1.0 - exp((abs(tv.y) - 1.0) * 3.0));
    color *= mix(0.0, 1.0, v);

	frag_color = vec4(color, 1.0);
}
