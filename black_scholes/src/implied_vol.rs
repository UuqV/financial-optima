use crate::black_scholes::{black_scholes, black_scholes_call, bs_deriv_k, d1, pdf};

pub fn newton(x0: f64, tol: f64, f: impl Fn(f64) -> f64, fprime: impl Fn(f64) -> f64) -> f64 {
    let mut xnew: f64 = x0;
    let mut xold: f64 = x0 - 1.0;
    while (xnew - xold).abs() > tol {
        xold = xnew;
        xnew = xold - f(xold) / fprime(xold);
        println!("{:#.8}", xnew);
    }
    return xnew;
}

pub fn implied_vol(c: f64, s: f64, k: f64, t: f64, r: f64, q: f64, x0: f64, tol: f64) -> f64 {
    return newton(
        x0,
        tol,
        |x| black_scholes_call(s, k, x, t, r, q) - c,
        |x| vega(s, k, x, t, r, q),
    );
}

fn vega(s: f64, k: f64, sigma: f64, t: f64, r: f64, q: f64) -> f64 {
    return s * pdf(d1(s, k, sigma, t, r, q)) * t.sqrt();
}

pub fn find_strike(
    s: f64,
    sigma: f64,
    t: f64,
    r: f64,
    q: f64,
    x0: f64,
    tol: f64,
    put: bool,
) -> f64 {
    return newton(
        x0,
        tol,
        |k| black_scholes(s, k, sigma, t, r, q, put) - k + s,
        |k| bs_deriv_k(s, k, sigma, t, r, q, put) - 1.0,
    );
}

#[cfg(test)]
mod black_scholes_test {
    use super::*;

    fn round(x: f64, power: f64) -> f64 {
        return (x * 10.0_f64.powf(power)).round() / (10.0_f64.powf(power));
    }
    #[test]
    fn newton_test() {
        assert_eq!(
            round(
                implied_vol(7.0, 25.0, 20.0, 1.0, 0.05, 0.0, 0.25, 0.000001),
                6.0
            ),
            0.363063
        );
    }
    #[test]
    fn vega_test() {
        assert_eq!(round(vega(25.0, 20.0, 0.20, 1.0, 0.05, 0.0), 6.0), 3.406799);
    }
    #[test]
    fn vega_q_test() {
        assert_eq!(
            round(vega(25.0, 20.0, 0.20, 1.0, 0.05, 0.01), 6.0),
            3.661266
        );
    }
    #[test]
    fn find_strike_call() {
        assert_eq!(
            round(
                find_strike(50.0, 0.25, 0.5, 0.03, 0.01, 50.0, 0.000001, false),
                6.0
            ),
            52.627583
        );
    }
}
