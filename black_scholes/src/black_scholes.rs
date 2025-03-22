use statrs::distribution::{ContinuousCDF, Normal};
use std::f64::consts::E;

pub fn black_scholes(s: f64, k: f64, sigma: f64, t: f64, r: f64) -> f64 {
    return k * E.powf(-r * t) * cdf(-d2(s, k, sigma, t, r)) - s * cdf(-d1(s, k, sigma, t, r));
}

pub fn delta(s: f64, k: f64, sigma: f64, t: f64, r: f64) -> f64 {
    return cdf(-d1(s, k, sigma, t, r));
}

fn d1(s: f64, k: f64, sigma: f64, t: f64, r: f64) -> f64 {
    return ((s / k).ln() + (r + (sigma.powf(2.0) / 2.0)) * t) / (sigma * t.sqrt());
}

fn d2(s: f64, k: f64, sigma: f64, t: f64, r: f64) -> f64 {
    return d1(s, k, sigma, t, r) - (sigma * t.sqrt());
}

fn cdf(x: f64) -> f64 {
    let n = Normal::new(0.0, 1.0).unwrap();
    return n.cdf(x);
}

fn rebalance(
    s: Vec<f64>,
    k: f64,
    sigma: f64,
    t: f64,
    r: f64,
    options: f64,
    asset: f64,
    cash: f64,
) -> f64 {
    println!("       Options     Asset        Cash  ");
    println!("Week 0 {:#.6} {:#.6} {:#.6}", options, asset, cash);
    for price in s.iter() {
        println!("The price is {:#.6}", price);
    }
    return k;
}

#[cfg(test)]
mod black_scholes_test {
    use super::*;

    fn round(x: f64, power: f64) -> f64 {
        return (x * 10.0_f64.powf(power)).round() / (10.0_f64.powf(power));
    }

    #[test]
    fn put_test() {
        assert_eq!(
            round(black_scholes(20.0, 25.0, 0.30, 0.5, 0.04), 6.0),
            4.927351
        );
    }
    #[test]
    fn delta_test() {
        assert_eq!(round(delta(20.0, 25.0, 0.30, 0.5, 0.04), 3.0), 0.803);
    }
    #[test]
    fn rebalance_test() {
        assert_eq!(
            round(
                rebalance(
                    vec![30.0, 26.0, 22.0, 27.0],
                    25.0,
                    0.30,
                    0.5,
                    0.04,
                    1000.0,
                    1265.8841,
                    -15394.46221
                ),
                6.0
            ),
            0.803
        );
    }
}
