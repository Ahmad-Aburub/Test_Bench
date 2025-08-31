use std::time::Instant;

const N: usize = 1000;

fn fill_matrix(n: usize) -> Vec<Vec<f64>> {
    let mut mat = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in 0..n {
            mat[i][j] = ((i * n + j) % 100) as f64;
        }
    }
    mat
}

fn matrix_mul(a: &[Vec<f64>], b: &[Vec<f64>], n: usize) -> Vec<Vec<f64>> {
    let mut c = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in 0..n {
            let mut sum = 0.0;
            for k in 0..n {
                sum += a[i][k] * b[k][j];
            }
            c[i][j] = sum;
        }
    }
    c
}

fn checksum(matrix: &[Vec<f64>]) -> f64 {
    matrix.iter().flatten().sum()
}

fn main() {
    let a = fill_matrix(N);
    let b = fill_matrix(N);

    let start = Instant::now();
    let c = matrix_mul(&a, &b, N);
    let duration = start.elapsed();

    let sum = checksum(&c);

    println!("Matrix size: {} x {}", N, N);
    println!("Time taken: {:.6} seconds", duration.as_secs_f64());
    println!("Result checksum: {:.2}", sum);
}
