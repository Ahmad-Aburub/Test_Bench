#!/home/ahmad/miniconda3/bin/python

import numpy as np
import time

N = 1000  # Same size as other versions

def create_matrix(n):
    return np.fromfunction(lambda i, j: (i * n + j) % 100, (n, n), dtype=int)

def main():
    a = create_matrix(N)
    b = create_matrix(N)

    start = time.perf_counter()
    c = a @ b  # or np.dot(a, b)
    end = time.perf_counter()

    checksum = np.sum(c)

    print(f"Matrix size: {N} x {N}")
    print(f"Time taken: {end - start:.6f} seconds")
    print(f"Result checksum: {checksum:.2f}")

if __name__ == "__main__":
    main()

