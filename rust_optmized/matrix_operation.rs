use std::time::Instant;

const N: usize = 1000;
const BLOCK_SIZE: usize = 64; // Tune for your CPU cache line size

fn fill_matrix(n: usize) -> Vec<f64> {
    let mut mat = vec![0.0; n * n];
    for i in 0..n {
        for j in 0..n {
            mat[i * n + j] = ((i * n + j) % 100) as f64;
        }
    }
    mat
}

// Naive multiply for reference (don't use in benchmark)
#[allow(dead_code)]
fn matmul_naive(a: &[f64], b: &[f64], c: &mut [f64], n: usize) {
    for i in 0..n {
        for j in 0..n {
            let mut sum = 0.0;
            for k in 0..n {
                sum += a[i * n + k] * b[k * n + j];
            }
            c[i * n + j] = sum;
        }
    }
}

// Blocked matrix multiplication for cache friendliness
fn matmul_blocked(a: &[f64], b: &[f64], c: &mut [f64], n: usize) {
    for ii in (0..n).step_by(BLOCK_SIZE) {
        for jj in (0..n).step_by(BLOCK_SIZE) {
            for kk in (0..n).step_by(BLOCK_SIZE) {
                let i_max = (ii + BLOCK_SIZE).min(n);
                let j_max = (jj + BLOCK_SIZE).min(n);
                let k_max = (kk + BLOCK_SIZE).min(n);

                for i in ii..i_max {
                    for j in jj..j_max {
                        let mut sum = c[i * n + j];
                        for k in kk..k_max {
                            sum += a[i * n + k] * b[k * n + j];
                        }
                        c[i * n + j] = sum;
                    }
                }
            }
        }
    }
}

fn checksum(c: &[f64]) -> f64 {
    c.iter().sum()
}

fn main() {
    let a = fill_matrix(N);
    let b = fill_matrix(N);
    let mut c = vec![0.0; N * N];

    let start = Instant::now();
    matmul_blocked(&a, &b, &mut c, N);
    let duration = start.elapsed();

    let sum = checksum(&c);

    println!("Matrix size: {} x {}", N, N);
    println!("Time taken: {:.6} seconds", duration.as_secs_f64());
    println!("Result checksum: {:.2}", sum);
}

