bool above_line(vec2 r, vec2 q, vec2 p) {
    return dot(vec2(q.y - r.y, r.x - q.x), q - p) > 0.0;
}
