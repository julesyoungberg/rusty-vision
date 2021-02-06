float RAND_SEED_;

void randSeed(vec2 c) {
    RAND_SEED_ = fract(sin(dot(c, vec2(113.421, 17.329))) * 3134.1234);
}

float rand() { return fract(sin(RAND_SEED_++) * 3143.45345); }
