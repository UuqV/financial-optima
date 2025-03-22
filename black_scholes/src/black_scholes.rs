use statrs::distribution::{ContinuousCDF, Normal};

pub fn cdf(x: f64) -> f64 {
    let n = Normal::new(0.0, 1.0).unwrap();
    return n.cdf(x);
}

pub fn round(x: f64, power: f64) -> f64 {
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
