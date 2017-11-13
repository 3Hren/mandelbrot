#version 150 core

uniform mat4 matrix;

out vec4 color;

const int iters = 100;

void main() {
    vec2 c = (matrix * vec4(gl_PointCoord, 0, 1)).xy;
    vec2 z = c;

    int i;
    for (i = 0; i < iters; i++) {
        z = vec2(pow(z.x, 2) - pow(z.y, 2), 2 * z.x * z.y) + c;
        if (dot(z, z) > 4.0) {
            break;
        }
    }

    float val = 1.0 - min(iters, float(i) / float(iters));
    color = vec4(val, val, val, 1.0);
}
