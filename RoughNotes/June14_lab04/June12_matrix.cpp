#include <emscripten/emscripten.h>
#include <vector>
#include <cstdlib>

// Flat array matrix multiply: C = A * B (all N x N)
extern "C" {

EMSCRIPTEN_KEEPALIVE
void matrix_multiply(
    float* A, float* B, float* C, int N)
{
    for (int i = 0; i < N; i++)
        for (int j = 0; j < N; j++) {
            float sum = 0.0f;
            for (int k = 0; k < N; k++)
                sum += A[i * N + k] * B[k * N + j];
            C[i * N + j] = sum;
        }
}

EMSCRIPTEN_KEEPALIVE
float* alloc_matrix(int N) {
    return (float*)malloc(N * N * sizeof(float));
}

EMSCRIPTEN_KEEPALIVE
void free_matrix(float* ptr) {
    free(ptr);
}

} // extern "C"
