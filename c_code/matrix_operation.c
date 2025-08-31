#include <stdio.h>
#include <stdlib.h>
#include <time.h>

#define N 1000  // Matrix size

void fill_matrix(double *mat, int n) {
    for (int i = 0; i < n * n; ++i)
        mat[i] = (double)(i % 100);
}

void print_matrix(const char *name, double *mat, int n) {
    printf("%s = [\n", name);
    for (int i = 0; i < n; ++i) {
        printf("  ");
        for (int j = 0; j < n; ++j)
            printf("%8.2f ", mat[i * n + j]);
        printf("\n");
    }
    printf("]\n\n");
}

void print_checksum(const char *name, double *mat, int n) {
    double sum = 0.0;
    for (int i = 0; i < n * n; ++i)
        sum += mat[i];
    printf("%s checksum: %.2f\n", name, sum);
}

void matrix_mul(double *a, double *b, double *c, int n) {
    for (int i = 0; i < n; ++i)
        for (int j = 0; j < n; ++j) {
            double sum = 0.0;
            for (int k = 0; k < n; ++k)
                sum += a[i * n + k] * b[k * n + j];
            c[i * n + j] = sum;
        }
}

double get_time_sec() {
    struct timespec t;
    clock_gettime(CLOCK_MONOTONIC, &t);
    return t.tv_sec + t.tv_nsec * 1e-9;
}

int main() {
    int n = N;

    double *a = malloc(sizeof(double) * n * n);
    double *b = malloc(sizeof(double) * n * n);
    double *c = malloc(sizeof(double) * n * n);

    fill_matrix(a, n);
    fill_matrix(b, n);

    // // 🖨️ Print input matrices (not timed)
    // print_matrix("Matrix A", a, n);
    // print_matrix("Matrix B", b, n);

    double start = get_time_sec();
    matrix_mul(a, b, c, n);
    double end = get_time_sec();

    printf("Matrix size: %d x %d\n", n, n);
    printf("Time taken: %.4f seconds\n", end - start);
    // print_matrix("Result Matrix C", c, n);
    print_checksum("Result", c, n);

    free(a);
    free(b);
    free(c);

    return 0;
}

