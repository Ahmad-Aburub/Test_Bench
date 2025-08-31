# 🧮 Matrix Multiplication Benchmark Across Languages

This project benchmarks large matrix multiplication across several programming languages:

- **C**
- **Fortran**
- **Rust** (with and without Rayon)
- **Python** (pure)
- **Python (NumPy)**

The goal is to compare performance, threading behavior, and numerical consistency across implementations.

---

## 🧪 Matrix Operation

The benchmark performs:

### C results:
- Matrix size: 1000 x 1000
- Time taken: 0.5082 seconds
- Result checksum: 2450250000000.00

### fortran results:
- Matrix size: 1000 x 1000
- Time taken:  0.42971399999999993 seconds
- Result checksum:   2450250000000.0000

### Base python results:
- Matrix size: 1000 x 1000
- Time taken: 42.468424 seconds
- Result checksum: 2450250000000.00

### Numpy Python results:
- Matrix size: 1000 x 1000
- Time taken: 0.481683 seconds
- Result checksum: 2450250000000.00

### Rust results:
- Matrix size: 1000 x 1000
- Time taken: 0.781608 seconds
- Result checksum: 2450250000000.00

### Rust Optimized/multithread:
Extra tests to show effect of reduced cache misses on the implementation as well as using multithreads
#### Optimized:
- Matrix size: 1000 x 1000
- Time taken: 0.474218 seconds
- Result checksum: 2450250000000.00
#### threads:
- Matrix size: 1000 x 1000
- Time taken (parallel): 0.066435 seconds
- Result checksum: 2450250000000.00
---
## 🧾 Build & Run Commands

### 🔧 C
```bash
gcc -O3 -o matrix_operation matrix_operation.c -lm
./matrix_operation
```
### 🔧 Fortran
```bash
gfortran -O3 -o matrix_operation matrix_operation.f90
./matrix_operation
```
### 🔧 python
```bash
python ./matrix_operation.py
```
### 🔧 rust
```bash
rustc -C opt-level=3 -C debuginfo=0 -C codegen-units=1 -C lto -C target-cpu=native matrix_operation.rs
./matrix_operation
```
## ⏱️ Benchmark Goals

- Compare **execution time** of equivalent operations
- Observe **multi-threading effects**
- Verify **numerical output consistency**
- Evaluate **language/runtime performance trade-offs**

---

## 🚀 Results Summary

| Language         | Threads Used     | Time (s) | Notes                          |
|------------------|------------------|----------|--------------------------------|
| C                | 1                | 0.5082      | Manual nested loops            |
| Fortran          | 1                | 0.429714      | Simple `do` loops              |
| Python (pure)    | 1                | 42.468424      | Nested list-based loops        |
| NumPy (1 thread) | 1                | 0.481683      | `OPENBLAS_NUM_THREADS=1`       |
| Rust (naive)     | 1                | 0.781608      | Basic implementation           |
| Rust (optmized)  | 1                | 0.474218      | Reduced Cache misses           |
| Rust + Rayon     | all cores        | 0.066435      | Blocked & parallelized version |

---
