use crate::black_scholes::{black_scholes, d1};
use statrs::distribution::{Continuous, Normal};

pub fn newton(x0: f64, tol: f64, f: impl Fn(f64) -> f64, fprime: impl Fn(f64) -> f64) -> f64 {
    let mut xnew: f64 = x0;
    let mut xold: f64 = x0 - 1.0;
    while (xnew - xold).abs() > tol {
        xold = xnew;
        xnew = xold + f(xold) / fprime(xold);
        println!("{:#.6}", xnew);
    }
    return xnew;
}

pub fn implied_vol(c: f64, s: f64, k: f64, t: f64, r: f64, x0: f64) -> f64 {
    return newton(
        x0,
        0.000001,
        |x| -1.0 * black_scholes(s, k, x, t, r) - c,
        |x| vega(s, k, x, t, r),
    );
}

fn pdf(x: f64) -> f64 {
    let n = Normal::new(0.0, 1.0).unwrap();
    return n.pdf(x);
}

fn vega(s: f64, k: f64, x: f64, t: f64, r: f64) -> f64 {
    return s * pdf(d1(s, k, x, t, r)) * t.sqrt();
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
            round(implied_vol(7.0, 25.0, 20.0, 1.0, 0.05, 0.25), 6.0),
            0.363063
        );
    }
    #[test]
    fn vega_test() {
        assert_eq!(round(vega(25.0, 20.0, 0.20, 1.0, 0.05), 6.0), 3.406799);
    }
}
