use std::f64::consts::E;
use std::f64::consts::PI;

pub fn standard_constants(accumulation: f64) -> f64 {
    return 0.5 + (1.0 / (2.0 * PI)).sqrt() * accumulation;
}

pub fn round(x: f64, power: f64) -> f64 {
    return (x * 10.0_f64.powf(power)).round() / (10.0_f64.powf(power));
}

pub fn standard_normal_variable(x: f64) -> f64 {
    return E.powf(-1.0 * (x.powf(2.0) / 2.0));
}

#[cfg(test)]
mod standard_normal_tests {
    use super::*;

    #[test]
    fn round_6_dec_test() {
        assert_eq!(round(E, 6.0), 2.718282);
    }
    #[test]
    fn round_12_dec_test() {
        assert_eq!(round(E, 12.0), 2.718281828459);
    }
    #[test]
    fn standard_constants_test() {
        assert_eq!(round(standard_constants(E), 6.0), 1.584438);
    }
    #[test]
    fn standard_normal_variable_test() {
        assert_eq!(round(standard_normal_variable(2.0), 6.0), 0.135335);
        assert_eq!(standard_normal_variable(0.0), 1.0);
    }
}
