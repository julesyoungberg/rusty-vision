#version 450

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 frag_color;

layout(set = 0, binding = 0) uniform Uniforms {
    int colorMode;
    int drawFloor;
    float fogDist;
    float time;
    vec2 resolution;
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

void main() {
    frag_color = vec4(abs(sin(time)), uv, 1);
}
