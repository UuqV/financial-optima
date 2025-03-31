mod lu;
use nalgebra::{matrix, vector, Dyn, OMatrix, OVector, LU};

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    // Define a square matrix A
    let a = OMatrix::<f64, Dyn, Dyn>::from_row_slice(
        3,
        3,
        &[4.0, -2.0, 1.0, 3.0, 6.0, -1.0, 2.0, 4.0, 3.0],
    );

    println!("{}", lu::build_ode(2.0 + (0.125), -1.0, 8));

    // Right-hand side vector b
    let b = OVector::<f64, Dyn>::from_vec(vec![1.0, 2.0, 3.0]);

    println!("{}", lu::decompose(a, b));

    let elapsed = now.elapsed();
    println!("\nElapsed: {:.2?}", elapsed);
}
