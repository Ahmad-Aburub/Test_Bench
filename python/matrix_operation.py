#!/home/ahmad/miniconda3/bin/python

import time

N = 1000


def create_matrix(n):
    return [[(i * n + j) % 100 for j in range(n)] for i in range(n)]

def matrix_mul(a, b, n):
    c = [[0.0 for _ in range(n)] for _ in range(n)]
    for i in range(n):
        for j in range(n):
            sum_ = 0.0
            for k in range(n):
                sum_ += a[i][k] * b[k][j]
            c[i][j] = sum_
    return c


def checksum(matrix):
    return sum(sum(row) for row in matrix)


def main():
    a = create_matrix(N)
    b = create_matrix(N)

    start = time.perf_counter()
    c = matrix_mul(a, b, N)
    end = time.perf_counter()

    print(f"Matrix size: {N} x {N}")
    print(f"Time taken: {end - start:.6f} seconds")
    print(f"Result checksum: {checksum(c):.2f}")

if __name__ == "__main__":
    main()

