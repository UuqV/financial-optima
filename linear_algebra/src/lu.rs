use nalgebra::{DMatrix, DVector, LU};
use std::f64::consts::E;

pub fn decompose() {
    // Define a square matrix A
    let a = DMatrix::from_row_slice(3, 3, &[4.0, -2.0, 1.0, 3.0, 6.0, -1.0, 2.0, 4.0, 3.0]);

    // Right-hand side vector b
    let b = DVector::from_vec(vec![1.0, 2.0, 3.0]);
    // Perform LU decomposition
    let lu = LU::new(a.clone());

    // Get the L and U matrices
    let l = lu.l();
    let u = lu.u();

    // Step 1: Solve Ly = b (forward substitution)
    let y = forward_substitution(&l, &b);

    // Step 2: Solve Ux = y (backward substitution)
    let x = backward_substitution(&u, &y);

    // Print the results
    println!("Matrix A (Original):");
    println!("{}", a);
    println!("Matrix L (Lower Triangular):");
    println!("{}", l);
    println!("Matrix U (Upper Triangular):");
    println!("{}", u);
    println!("Solution vector x:");
    println!("{}", x);
}

fn forward_substitution(l: &DMatrix<f64>, b: &DVector<f64>) -> DVector<f64> {
    let n = l.nrows();
    let mut y = DVector::zeros(n);

    for i in 0..n {
        let mut sum = 0.0;
        for j in 0..i {
            sum += l[(i, j)] * y[j];
        }
        y[i] = (b[i] - sum) / l[(i, i)];
    }

    y
}

fn backward_substitution(u: &DMatrix<f64>, y: &DVector<f64>) -> DVector<f64> {
    let n = u.nrows();
    let mut x = DVector::zeros(n);

    for i in (0..n).rev() {
        let mut sum = 0.0;
        for j in i + 1..n {
            sum += u[(i, j)] * x[j];
        }
        x[i] = (y[i] - sum) / u[(i, i)];
    }

    x
}

#[cfg(test)]
mod lu_test {
    use super::*;

    fn round(x: f64, power: f64) -> f64 {
        return (x * 10.0_f64.powf(power)).round() / (10.0_f64.powf(power));
    }
}
