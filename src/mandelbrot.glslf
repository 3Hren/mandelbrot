#version 150 core

uniform mat4 matrix;

out vec4 color;

const int threshold = 200;

bool fast_check(vec2 z) {
    float r = sqrt(pow(z.x - 0.25, 2) + pow(z.y, 2));
    if (z.x < r - 2 * pow(r, 2) + 0.25) {
        color = vec4(0, 0, 0, 1.0);
        return false;
    }

    if (pow(z.x + 1, 2) + pow(z.y, 2) < 0.0625) {
        color = vec4(0, 0, 0, 1.0);
        return false;
    }

    return true;
}

vec3 hsv2rgb(vec3 c) {
    vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
    return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}

vec4 blackwhite(int i) {
    float n = 1.0 - min(threshold, float(i) / float(threshold));
    return vec4(n, n, n, 1.0);
}

vec4 grayscale(int i) {
    if (i == threshold) {
        return vec4(0.0, 0.0, 0.0, 1.0);
    } else {
        float n = sqrt(float(i) / float(threshold));
        n *= 1.0;
        return vec4(n, n, n, 1.0);
    }
}

void main() {
    vec2 c = (matrix * vec4(gl_PointCoord, 0, 1)).xy;
    vec2 z = c;

    int i;
    for (i = 0; i < threshold; i++) {
        z = vec2(pow(z.x, 2) - pow(z.y, 2), 2 * z.x * z.y) + c;
        if (dot(z, z) > 4.0) {
            break;
        }
    }

    color = grayscale(i);
}
