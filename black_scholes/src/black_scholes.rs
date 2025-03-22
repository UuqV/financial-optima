use statrs::distribution::{ContinuousCDF, Normal};

pub fn black_scholes(s: f64, k: f64, sigma: f64, t: f64) -> f64 {
    return s;
}

fn cdf(x: f64) -> f64 {
    let n = Normal::new(0.0, 1.0).unwrap();
    return n.cdf(x);
}

fn round(x: f64, power: f64) -> f64 {
    return (x * 10.0_f64.powf(power)).round() / (10.0_f64.powf(power));
}

#[cfg(test)]
mod black_scholes_test {
    use super::*;

    #[test]
    fn dist_test() {
        assert_eq!(round(cdf(0.5), 4.0), 0.6915);
    }
}
