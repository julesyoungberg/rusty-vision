// IQ's impulse: https://www.iquilezles.org/www/articles/functions/functions.htm
float impulse(float k, float n, float x) {
    return (n / (n - 1.0)) * pow((n - 1.0) * k, 1.0 / n) * x /
           (1.0 + k * pow(x, n));
}
