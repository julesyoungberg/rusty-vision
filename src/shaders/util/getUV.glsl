vec2 getUV(in vec2 coord, in vec2 res) {
    vec2 uv = coord / res;
    uv = uv * 2.0 - 1.0;
    uv.x *= res.x / res.y;
    return uv;
}
