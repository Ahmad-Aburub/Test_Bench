use rayon::prelude::*;
use std::time::Instant;

const N: usize = 1000;
const BLOCK_SIZE: usize = 64;

fn fill_matrix(n: usize) -> Vec<f64> {
    let mut mat = vec![0.0; n * n];
    for i in 0..n {
        for j in 0..n {
            mat[i * n + j] = ((i * n + j) % 100) as f64;
        }
    }
    mat
}

fn matmul_blocked_parallel(a: &[f64], b: &[f64], c: &mut [f64], n: usize) {
    let block_rows = BLOCK_SIZE;

    c.par_chunks_mut(block_rows * n)
        .enumerate()
        .for_each(|(block_i, c_block)| {
            let ii = block_i * block_rows;
            let i_max = (ii + block_rows).min(n);

            for jj in (0..n).step_by(BLOCK_SIZE) {
                for kk in (0..n).step_by(BLOCK_SIZE) {
                    let j_max = (jj + BLOCK_SIZE).min(n);
                    let k_max = (kk + BLOCK_SIZE).min(n);

                    for i in 0..(i_max - ii) {
                        for j in jj..j_max {
                            let mut sum = c_block[i * n + j];
                            for k in kk..k_max {
                                sum += a[(ii + i) * n + k] * b[k * n + j];
                            }
                            c_block[i * n + j] = sum;
                        }
                    }
                }
            }
        });
}

fn checksum(c: &[f64]) -> f64 {
    c.iter().sum()
}

fn main() {
    let a = fill_matrix(N);
    let b = fill_matrix(N);
    let mut c = vec![0.0; N * N];

    let start = Instant::now();
    matmul_blocked_parallel(&a, &b, &mut c, N);
    let duration = start.elapsed();

    let sum = checksum(&c);

    println!("Matrix size: {} x {}", N, N);
    println!("Time taken (parallel): {:.6} seconds", duration.as_secs_f64());
    println!("Result checksum: {:.2}", sum);
}

