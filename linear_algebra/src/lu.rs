use nalgebra::{matrix, vector, Dyn, OMatrix, OVector, LU};
use std::f64::consts::E;

pub fn decompose(a: OMatrix<f64, Dyn, Dyn>, b: OVector<f64, Dyn>) -> OVector<f64, Dyn> {
    println!("OMatrix A (Original):");
    println!("{}", a);
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
    println!("OMatrix L (Lower Triangular):");
    println!("{}", l);
    println!("OMatrix U (Upper Triangular):");
    println!("{}", u);
    println!("Solution vector x:");
    println!("{}", x);
    return x;
}

fn forward_substitution(l: &OMatrix<f64, Dyn, Dyn>, b: &OVector<f64, Dyn>) -> OVector<f64, Dyn> {
    let n = l.nrows();
    let mut y: OVector<f64, Dyn> = OVector::<f64, Dyn>::zeros(n);

    for i in 0..n {
        let mut sum = 0.0;
        for j in 0..i {
            sum += l[(i, j)] * y[j];
        }
        y[i] = (b[i] - sum) / l[(i, i)];
    }

    y
}

fn backward_substitution(u: &OMatrix<f64, Dyn, Dyn>, y: &OVector<f64, Dyn>) -> OVector<f64, Dyn> {
    let n = u.nrows();
    let mut x: OVector<f64, Dyn> = OVector::<f64, Dyn>::zeros(n);

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
    #[test]
    fn toy() {
        // Define a square matrix A
        let a = OMatrix::<f64, Dyn, Dyn>::from_row_slice(
            3,
            3,
            &[4.0, -2.0, 1.0, 3.0, 6.0, -1.0, 2.0, 4.0, 3.0],
        );

        // Right-hand side vector b
        let b = OVector::<f64, Dyn>::from_vec(vec![1.0, 2.0, 3.0]);

        println!("{}", decompose(a, b));
    }
}
