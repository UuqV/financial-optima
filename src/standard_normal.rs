use std::f64::consts::E;
use std::f64::consts::PI;

pub fn standard_constants(accumulation: f64) -> f64 {
    return 0.5 + (1.0 / (2.0 * PI)).sqrt() * accumulation;
}

pub fn round_6_dec(x: f64) -> f64 {
    return (x * 1000000.0).round() / 1000000.0;
}

pub fn standard_normal_variable(x: f64) -> f64 {
    return E.powf(-1.0 * (x.powf(2.0) / 2.0));
}

#[cfg(test)]
mod standard_normal_tests {
    use super::*;

    #[test]
    fn round_6_dec_test() {
        assert_eq!(round_6_dec(E), 2.718282);
    }
    #[test]
    fn standard_constants_test() {
        assert_eq!(round_6_dec(standard_constants(E)), 1.584438);
    }
    #[test]
    fn standard_normal_variable_test() {
        assert_eq!(round_6_dec(standard_normal_variable(2.0)), 0.135335);
    }
}
