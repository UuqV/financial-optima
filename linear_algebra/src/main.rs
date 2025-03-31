mod lu;
use nalgebra::{matrix, vector, Dyn, OMatrix, OVector, LU};
use std::f64::consts::E;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let a = lu::build_ode(
        |h: f64| 2.0 - h.powf(2.0),
        |h: f64| -1.0 * (1.0 - h.powf(2.0)),
        0.125,
        7,
    );

    println!("{}", a);

    let b = lu::build_solution(
        |h: f64| 2.0 * (1.0 - h),
        |h: f64| (3.0 / E) * (1.0 + h),
        0.125,
        7,
    );

    println!("{}", b);

    println!("{}", lu::decompose(a, b));

    let elapsed = now.elapsed();
    println!("\nElapsed: {:.2?}", elapsed);
}

fn book() {
    // Define a square matrix A
    let a = lu::build_ode(|h: f64| 2.0 + h.powf(2.0), |h: f64| -1.0, 0.125, 7);

    println!("{}", a);

    // Right-hand side vector b
    let b = lu::build_solution(|h: f64| 1.0, |h: f64| E, 0.125, 7);

    println!("{}", b);

    println!("{}", lu::decompose(a, b));
}
