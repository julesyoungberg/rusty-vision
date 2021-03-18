// IQ's pulse function
// https://www.iquilezles.org/www/articles/functions/functions.htm
float pulse(float c, float w, float x) {
    x = abs(x - c);
    if (x > w)
        return 0.0;
    x /= w;
    return 1.0 - x * x * (3.0 - 2.0 * x);
}
