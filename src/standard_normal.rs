use std::f64::consts::E;
use std::f64::consts::PI;

pub fn standard_constants(accumulation: f64) -> f64 {
    return 0.5 + (1.0 / (2.0 * PI)).sqrt() * accumulation;
}

pub fn round_6_dec(number: f64) -> f64 {
    return (number * 1000000.0).round() / 1000000.0;
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
}
