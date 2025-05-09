use nalgebra::{matrix, vector, Dyn, OMatrix, OVector, LU};
use std::f64::consts::E;

pub fn decompose(a: OMatrix<f64, Dyn, Dyn>, b: OVector<f64, Dyn>) -> OVector<f64, Dyn> {
    let lu = LU::new(a.clone());

    let l = lu.l();
    let u = lu.u();

    let y = forward_substitution(&l, &b);

    let x = backward_substitution(&u, &y);

    return x;
}

pub fn power_intervals(n: u32) {
    for r in 3..=n {
        let power = 2_u32.pow(r) as usize;
        let h = 1.0 / power as f64;
        println!("\nn = {}, h = {}\n", power, h);
        let a = build_ode(
            |h: f64| 2.0 - h.powf(2.0),
            |h: f64| -1.0 * (1.0 - h),
            |h: f64| -1.0 * (1.0 + h),
            h,
            power,
        );

        let b = build_solution(
            |h: f64| 2.0 * (1.0 - h),
            |h: f64| (3.0 / E) * (1.0 + h),
            h,
            power,
        );

        let solution = decompose(a, b);

        println!(
            "       {:width$} {:width$} {:width$} {:width$}",
            "xi",
            "yi",
            "y(xi)",
            "|yi - y(xi)|",
            width = 15
        );
        let mut x = 0.0;
        for i in 1..power {
            x += h;
            println!(
                "{:width$.6} {:width$.6} {:width$.6} {:width$.6}",
                x,
                solution[i - 1],
                2.0 * E.powf(-x) + x * E.powf(-x),
                solution[i - 1] - (2.0 * E.powf(-x) + x * E.powf(-x)),
                width = 15
            );
        }
    }
}

pub fn build_ode(
    i: fn(h: f64) -> f64,
    pi: fn(h: f64) -> f64,
    ni: fn(h: f64) -> f64,
    h: f64,
    n: usize,
) -> OMatrix<f64, Dyn, Dyn> {
    let mut a: OMatrix<f64, Dyn, Dyn> = OMatrix::<f64, Dyn, Dyn>::zeros(n - 1, n - 1);

    a[(0, 0)] = i(h);
    for r in 1..(n - 1) {
        a[(r, r)] = i(h);
        a[(r, r - 1)] = pi(h);
        a[(r - 1, r)] = ni(h);
    }

    return a;
}

pub fn build_solution(
    start: fn(h: f64) -> f64,
    end: fn(h: f64) -> f64,
    h: f64,
    n: usize,
) -> OVector<f64, Dyn> {
    let mut b: OVector<f64, Dyn> = OVector::<f64, Dyn>::zeros(n - 1);

    b[0] = start(h);
    b[n - 2] = end(h);

    return b;
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

        assert_eq!(
            OVector::<f64, Dyn>::from_vec(vec![
                0.2727272727272727,
                0.2727272727272727,
                0.4545454545454546
            ]),
            decompose(a, b)
        );
    }
    fn book() {}
}
