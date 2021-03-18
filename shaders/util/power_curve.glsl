// IQ's power curve function
// https://www.iquilezles.org/www/articles/functions/functions.htm
float power_curve(float x, float a, float b) {
    const float k = pow(a + b, a + b) / (pow(a, a) * pow(b, b));
    return k * pow(x, a) * pow(1.0 - x, b);
}
